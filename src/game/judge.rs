use crate::{cache::CachedCall, database::Database, CONFIG};
use anyhow::{Context, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema},
    Client as OpenAIClient,
};
use axum::{extract::Request, response::IntoResponse, Extension};
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use twilio::{twiml::Twiml, Call, Client as TwilioClient, OutboundMessage};

pub async fn judge_handler(
    twilio: Extension<TwilioClient>,
    openai: Extension<OpenAIClient<OpenAIConfig>>,
    cache: Extension<Arc<Mutex<HashMap<String, CachedCall>>>>,
    database: Extension<Database>,
    request: Request,
) -> impl IntoResponse {
    twilio
        .clone()
        .respond_to_webhook_async(request, |call: Call| async move {
            let cached_call = {
                let mut cache = cache.lock().await;
                let cached_call = cache
                    .get_mut(&call.sid)
                    .expect("Failed to get message conversation");

                cached_call.end_last_message();
                cached_call.clone()
            };

            cached_call.write_subtitles_to_file(&call.sid);
            tokio::spawn(judge_conversation(
                twilio.0,
                call.from,
                openai.0,
                database.0,
                cached_call,
            ));

            Twiml::new()
        })
        .await
}

#[derive(Debug, Deserialize)]
pub struct JudgeResponse {
    pub won_prize: bool,
}

async fn judge_conversation(
    twilio: TwilioClient,
    caller_phone_number: String,
    openai: OpenAIClient<OpenAIConfig>,
    database: Database,
    cached_call: CachedCall,
) {
    let schema = json!({
        "type": "object",
        "properties": {
            "won_prize": {
                "type": "boolean",
                "description": CONFIG.end.schema_property
            }
        },
        "required": ["won_prize"],
        "additionalProperties": false,
    });

    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: Some(CONFIG.end.schema_description.to_owned()),
            name: "won_prize_extraction".to_owned(),
            schema: Some(schema),
            strict: Some(true),
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(CONFIG.end.max_tokens as u32)
        .model(CONFIG.end.model)
        .messages(cached_call.messages.clone())
        .response_format(response_format)
        .build()
        .expect("Failed to build chat completion request");

    let response = openai
        .chat()
        .create(request)
        .await
        .expect("Failed to create chat completion");
    let choice = response
        .choices
        .first()
        .expect("Failed to get first choice");
    let content = choice
        .message
        .content
        .as_ref()
        .expect("Failed to get content");

    let judged: JudgeResponse =
        serde_json::from_str(&content).expect("Failed to judge conversation");

    let result = match judged.won_prize {
        true => won_handler(twilio, database, caller_phone_number, cached_call).await,
        false => lost_handler(twilio, database, caller_phone_number, cached_call).await,
    };

    if let Err(e) = result {
        log::error!("Failed to handle call judge result: {e:?}");
    }
}

async fn won_handler(
    twilio: TwilioClient,
    database: Database,
    caller_phone_number: String,
    cached_call: CachedCall,
) -> Result<()> {
    log::debug!("Won prize for sponsor: {}", cached_call.sponsor.name);

    // Withdraw tokens from the sponsor
    let withdrawn = database
        .withdraw_tokens(cached_call.sponsor.id)
        .await
        .context("Withdrawing tokens")?;

    // If withdrawing tokens failed, redirect to lost handler
    if withdrawn.is_none() {
        return lost_handler(twilio, database, caller_phone_number, cached_call).await;
    };

    // Generate a winner entry in the database
    let winner = database
        .create_winner(cached_call.name.clone(), cached_call.sponsor.id)
        .await
        .context("Creating winner")?;

    // Generate the winning link
    let link = format!(
        "{}/claim?key={}",
        CONFIG.settings.global_address, winner.key
    );

    // Generate the winning text
    let text = cached_call
        .sponsor
        .won_text
        .replace("{name}", &cached_call.name)
        .replace("{link}", &link);

    twilio
        .send_message(OutboundMessage {
            from: CONFIG.settings.phone_number,
            to: &caller_phone_number,
            body: &text,
        })
        .await
        .context("Sending message")?;

    Ok(())
}

async fn lost_handler(
    twilio: TwilioClient,
    _database: Database,
    caller_phone_number: String,
    cached_call: CachedCall,
) -> Result<()> {
    log::debug!("Lost prize for sponsor: {}", cached_call.sponsor.name);

    // Generate the loosing text
    let text = cached_call
        .sponsor
        .lost_text
        .replace("{name}", &cached_call.name);

    twilio
        .send_message(OutboundMessage {
            from: CONFIG.settings.phone_number,
            to: &caller_phone_number,
            body: &text,
        })
        .await
        .context("Sending message")?;

    Ok(())
}

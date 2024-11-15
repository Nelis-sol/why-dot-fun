use crate::{cache::CachedCall, CONFIG};
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
use twilio::{twiml::Twiml, Call, Client, OutboundMessage};

pub async fn judge_handler(
    twilio: Extension<Client>,
    openai: Extension<OpenAIClient<OpenAIConfig>>,
    cache: Extension<Arc<Mutex<HashMap<String, CachedCall>>>>,
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
            tokio::spawn(judge_conversation(twilio, call.from, openai, cached_call));

            Twiml::new()
        })
        .await
}

#[derive(Debug, Deserialize)]
pub struct JudgeResponse {
    pub won_prize: bool,
}

async fn judge_conversation(
    twilio: Extension<Client>,
    caller_phone_number: String,
    openai: Extension<OpenAIClient<OpenAIConfig>>,
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
        .messages(cached_call.messages)
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

    twilio
        .send_message(OutboundMessage {
            from: CONFIG.settings.phone_number,
            to: &caller_phone_number,
            body: match judged.won_prize {
                true => &cached_call.sponsor.won_text,
                false => &cached_call.sponsor.lost_text,
            },
        })
        .await
        .expect("Failed to send message");
}

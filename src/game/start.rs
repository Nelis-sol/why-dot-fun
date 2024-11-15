use crate::{
    cache::CachedCall,
    database::{Database, Sponsor},
    CONFIG,
};
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
};
use axum::{extract::Request, response::IntoResponse, Extension};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use twilio::{
    twiml::{Method, Redirect, Say, Twiml, Voice},
    Call, Client as TwilioClient,
};

pub async fn start_handler(
    twilio: Extension<TwilioClient>,
    cache: Extension<Arc<Mutex<HashMap<String, CachedCall>>>>,
    database: Extension<Database>,
    request: Request,
) -> impl IntoResponse {
    twilio
        .clone()
        .respond_to_webhook_async(request, |call: Call| async move {
            log::debug!("Received call from {} with id {}", call.from, call.sid);

            // Get the sponsor for the call
            let sponsor = database
                .get_random_sponsor()
                .await
                .expect("Failed to get sponsor");

            // Generate the twiml response
            let twiml = generate_start_twiml(&sponsor.greeting_text);

            // Add the call to the cache
            initialize_cached_call(&cache, call.sid.clone(), sponsor).await;

            // Start call recording
            tokio::spawn(start_call_recording(twilio.0, call.sid.clone()));

            twiml
        })
        .await
}

/// Generate the TwiML for the start of the call.
/// 1. Greet the user
/// 2. Redirect to the /name route to start the name query process
fn generate_start_twiml(greeting: &str) -> Twiml {
    let mut twiml = Twiml::new();

    twiml.add(&Say {
        txt: greeting.to_owned(),
        voice: Voice::Custom(CONFIG.settings.voice.to_owned()),
        language: CONFIG.settings.language.to_owned(),
    });

    twiml.add(&Redirect {
        method: Method::Post,
        url: "/redirect-gather/name".to_owned(),
    });

    twiml
}

/// Initialize the conversation cache with two messages:
/// - The system message with the sponsor's system instruction
/// - The assistant message with the sponsor's greeting text
///
/// The system message is not audible and ignored by the subtitle
/// generation, it only serves to instruct the model on how to respond.
async fn initialize_cached_call(
    cache: &Arc<Mutex<HashMap<String, CachedCall>>>,
    call_sid: String,
    sponsor: Sponsor,
) {
    let mut cached_call = CachedCall::new(sponsor.clone());
    cached_call.add_system_message(
        ChatCompletionRequestSystemMessageArgs::default()
            .content(sponsor.system_instruction)
            .build()
            .expect("Failed to build system message")
            .into(),
    );
    cached_call.add_system_message(
        ChatCompletionRequestAssistantMessageArgs::default()
            .content(sponsor.greeting_text)
            .build()
            .expect("Failed to build system message")
            .into(),
    );

    cache.lock().await.insert(call_sid, cached_call);
}

/// Start the call recording. The recording may fail to start if the call status
/// on twilio's backend has not yet updated to `in-progress`. In this case, the
/// recording will be retried a number of times before giving up. This is a known
/// limitation of the Twilio API and the recommended approach by twilio evangelists.
async fn start_call_recording(twilio: TwilioClient, call_sid: String) {
    for _ in 0..CONFIG.settings.record_retry {
        if let Ok(recording) = twilio
            .record_call(
                &call_sid,
                &format!("{}/recording", CONFIG.settings.global_address),
            )
            .await
        {
            log::debug!("Recording started with id {}", recording.sid);
            return;
        }
    }

    log::error!(
        "Failed to start recording after {} retries",
        CONFIG.settings.record_retry
    );
}

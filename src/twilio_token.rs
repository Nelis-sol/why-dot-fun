use crate::secrets::Secrets;
use axum::Extension;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

#[derive(Serialize)]
struct Grants {
    identity: String,
    voice: VoiceGrant,
}

#[derive(Serialize)]
struct VoiceGrant {
    incoming: IncomingVoiceGrant,
    outgoing: OutgoingVoiceGrant,
}

#[derive(Serialize)]
struct IncomingVoiceGrant {
    allow: bool,
}

#[derive(Serialize)]
struct OutgoingVoiceGrant {
    application_sid: String,
}

#[derive(Serialize)]
struct Claims {
    jti: String,
    iss: String,
    sub: String,
    iat: i64,
    nbf: i64,
    exp: i64,
    grants: Grants,
}

pub async fn generate_jwt(secrets: Extension<Secrets>) -> String {
    // Set current time and expiration
    let now = Utc::now();
    let iat = now.timestamp();
    let nbf = iat;
    let exp = (now + Duration::seconds(1600)).timestamp();

    // Build grants
    let grants = Grants {
        identity: "webapp".to_string(),
        voice: VoiceGrant {
            incoming: IncomingVoiceGrant { allow: false },
            outgoing: OutgoingVoiceGrant {
                application_sid: secrets.twilio_app_sid.clone(),
            },
        },
    };

    // Build claims
    let claims = Claims {
        jti: format!("{}-{}", secrets.twilio_api_key, iat),
        iss: secrets.twilio_api_key.clone(),
        sub: secrets.twilio_account_sid.clone(),
        iat,
        nbf,
        exp,
        grants,
    };

    // Create header
    let header = Header {
        cty: Some("twilio-fpa;v=1".to_string()),
        ..Default::default()
    };

    // Encode the token
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secrets.twilio_api_secret.as_bytes()),
    )
    .expect("Failed to generate JWT")
}

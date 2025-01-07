use axum::response::IntoResponse;
use axum::Json;
use crate::api::{Attempt, AttemptReturn};
use axum::Extension;
use crate::Database;
use serde::Deserialize;
use solana_sdk::signature::{Signature, Signer};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use crate::StatusCode;


#[derive(Deserialize, Clone, Debug)]
pub struct SponsorListArgs {
    public_key: String,
    signature: String,
}

pub async fn sponsor_list(
    Extension(database): Extension<Database>,
    Json(request): Json<SponsorListArgs>,
) -> impl IntoResponse {
    println!("sponsor list request: {:?}", request.clone());

    let signature = request.signature;
    let public_key = request.public_key;

    // Convert the signature and public key from strings to their respective types
    let signature = Signature::from_str(&signature).expect("Invalid signature format");
    let public_key = Pubkey::from_str(&public_key).expect("Invalid public key format");

    let message = chrono::Utc::now().format("%Y-%m-%d %H:00:00").to_string();
    println!("message: {}", message);

    // Verify the signature
    if !signature.verify(&public_key.to_bytes(), message.as_bytes()) {
        return (StatusCode::BAD_REQUEST, Json("Invalid signature")).into_response();
    }

    let sponsor_list = database
        .get_sponsor_by_user_id(public_key.to_string())
        .await
        .expect("Failed to get sponsor");

        
    (StatusCode::OK, Json(sponsor_list)).into_response()
}

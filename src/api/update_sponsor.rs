use axum::response::IntoResponse;
use axum::Json;
use crate::api::{Attempt, AttemptReturn};
use axum::Extension;
use crate::Database;
use serde::Deserialize;
use solana_sdk::signature::{Signature, Signer};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use crate::database::Sponsor;
use crate::StatusCode;

use super::SponsorArgs;


#[derive(Deserialize)]
pub struct UpdateSponsorArgs {
    public_key: String,
    signature: String,
    sponsor: Sponsor,
}

pub async fn update_sponsor(
    Extension(database): Extension<Database>,
    Json(request): Json<SponsorArgs>,
) -> impl IntoResponse {

    println!("update sponsor request: {:?}", request.clone());

    let signature = request.transaction;
    let public_key = request.user_id;

    // Convert the signature and public key from strings to their respective types
    let signature = Signature::from_str(&signature).expect("Invalid signature format");
    let public_key = Pubkey::from_str(&public_key).expect("Invalid public key format");

    let message = chrono::Utc::now().format("%Y-%m-%d %H:00:00").to_string();
    println!("current_hour: {}", message);

    // Verify the signature
    if !signature.verify(&public_key.to_bytes(), message.as_bytes()) {
        return (StatusCode::BAD_REQUEST, Json("Invalid signature")).into_response();
    }

    (StatusCode::OK).into_response()
}
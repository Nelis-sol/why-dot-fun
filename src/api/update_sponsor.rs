use axum::response::IntoResponse;
use axum::Json;
use crate::api::{Attempt, AttemptReturn};
use axum::Extension;
use crate::Database;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::{Signature, Signer};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use crate::api::ReturnSponsor;
use crate::StatusCode;


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateSponsorArgs {
    pub public_key: String,
    pub name: String,
    pub active: bool,
    pub background_url: String,
    pub token_mint: String,
    pub challenge_time: i32,
    pub system_instruction: String,
    pub greeting_text: String,
    pub start_text: String,
    pub end_text: String,
    pub won_text: String,
    pub lost_text: String,
    pub rating_threshold: i32,
    pub challenge_text: String,
    pub user_id: String,
    pub signature: String
}

pub async fn update_sponsor(
    Extension(database): Extension<Database>,
    Json(request): Json<UpdateSponsorArgs>,
) -> impl IntoResponse {

    println!("update sponsor request: {:?}", request.clone());

    let signature = &request.signature;
    let public_key = &request.user_id;

    // Convert the signature and public key from strings to their respective types
    let signature = Signature::from_str(signature).expect("Invalid signature format");
    let public_key = Pubkey::from_str(public_key).expect("Invalid public key format");

    let message = chrono::Utc::now().format("%Y-%m-%d %H:00:00").to_string();
    println!("current_hour: {}", message);

    // Verify the signature
    if !signature.verify(&public_key.to_bytes(), message.as_bytes()) {
        return (StatusCode::BAD_REQUEST, Json("Invalid signature")).into_response();
    }

    let sponsor_entry = database.update_sponsor(request)
        .await
        .expect("Failed to update sponsor");


    let return_sponsor = ReturnSponsor {
        id: sponsor_entry.id,
        name: sponsor_entry.name,
        user_id: sponsor_entry.user_id,
        active: sponsor_entry.active,
        background_url: sponsor_entry.background_url,
        public_key: sponsor_entry.public_key,
        token_mint: sponsor_entry.token_mint,
        original_tokens: sponsor_entry.original_tokens,
        available_tokens: sponsor_entry.available_tokens,
        reward_tokens: sponsor_entry.reward_tokens,
        challenge_text: sponsor_entry.challenge_text,
        challenge_time: sponsor_entry.challenge_time,
        system_instruction: sponsor_entry.system_instruction,
        start_text: sponsor_entry.start_text,
        won_text: sponsor_entry.won_text,
        lost_text: sponsor_entry.lost_text,
        rating_threshold: sponsor_entry.rating_threshold,
    };

    (StatusCode::OK, Json(return_sponsor)).into_response()
}
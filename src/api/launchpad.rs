use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::database::Sponsor;
use crate::api::SponsorArgs;
use crate::Database;
use anyhow::Context;
use crate::StatusCode;
use serde::Serialize;
use crate::solana::keys::generate_private_key;
use solana_sdk::signer::Signer;

#[derive(Serialize)]
pub struct ReturnSponsor {
    id: i32,
    name: String,
    active: bool,
    background_url: String,
    public_key: String,
    token_mint: String,
    original_tokens: i32,
    available_tokens: i32,
    reward_tokens: i32,
    challenge_time: i32,
    start_text: String,
    system_instruction: String,
    won_text: String,
    lost_text: String,
    rating_threshold: i32,
}

pub async fn launchpad(
    Extension(database): Extension<Database>,
    Json(new_sponsor): Json<SponsorArgs>,
) -> impl IntoResponse {
    let challenge: String = String::from("Thank you {name}! Lets start the game. You have {duration} seconds to answer the following question: ");

    let private_key = generate_private_key();
    let public_key = private_key.pubkey().to_string();
    let private_key_base58 = private_key.to_base58_string();

    let sponsor = Sponsor {
        id: 1,
        name: new_sponsor.name,
        active: true,
        background_url: new_sponsor.background_url,
        private_key: private_key_base58,
        public_key: public_key.to_string(),
        token_mint: new_sponsor.token_mint,
        original_tokens: new_sponsor.original_tokens,
        available_tokens: new_sponsor.available_tokens,
        reward_tokens: new_sponsor.reward_tokens,
        challenge_time: new_sponsor.challenge_time,
        system_instruction: new_sponsor.system_instruction,
        greeting_text: "Welcome to Why dot Fun. Please tell me your name to start the game.".to_string(),
        start_text: format!("{} {}", challenge, new_sponsor.start_text),
        end_text: "Alright, your time is up! Thank you for participating. You will receive a text message with the results of you attempt. Thank you for playing today!".to_string(),
        won_text: "Congratulations, you won! Claim you prize: https://www.why.fun/crab?winner=SdfIjwfdsoBYNOUufd".to_string(),
        lost_text: "Unfortunately, you lost the game. Better luck next time!".to_string(),
        rating_threshold: new_sponsor.rating_threshold,
    };

    let sponsor_entry = database
        .create_sponsor(sponsor)
        .await
        .context("Creating sponsor")
        .expect("Failed to create sponsor");

    let return_sponsor = ReturnSponsor {
        id: sponsor_entry.id,
        name: sponsor_entry.name,
        active: sponsor_entry.active,
        background_url: sponsor_entry.background_url,
        public_key: sponsor_entry.public_key,
        token_mint: sponsor_entry.token_mint,
        original_tokens: sponsor_entry.original_tokens,
        available_tokens: sponsor_entry.available_tokens,
        reward_tokens: sponsor_entry.reward_tokens,
        challenge_time: sponsor_entry.challenge_time,
        system_instruction: sponsor_entry.system_instruction,
        start_text: sponsor_entry.start_text,
        won_text: sponsor_entry.won_text,
        lost_text: sponsor_entry.lost_text,
        rating_threshold: sponsor_entry.rating_threshold,
    };

    let response = (StatusCode::CREATED, Json(return_sponsor)).into_response();
    response
}

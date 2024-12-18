use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::database::Sponsor;
use crate::api::SponsorArgs;
use crate::Database;
use anyhow::Context;
use crate::StatusCode;


pub async fn launchpad(
    database: Extension<Database>,
    Json(new_sponsor): Json<SponsorArgs>,
) -> impl IntoResponse {
    let challenge: String = String::from("Thank you {name}! Lets start the game. You have {duration} seconds to answer the following question: ");

    let sponsor = Sponsor {
        id: 1,
        name: new_sponsor.name,
        active: true,
        background_url: new_sponsor.background_url,
        private_key: "test_private_key".to_string(),
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

    database
        .create_sponsor(sponsor)
        .await
        .context("Creating sponsor")
        .expect("Failed to create sponsor");

    StatusCode::OK
}
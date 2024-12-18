use axum::response::IntoResponse;
use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchpadArgs {
    pub name: String,
    pub background_url: String,
    pub token_mint: String,
    pub original_tokens: i32,
    pub available_tokens: i32,
    pub reward_tokens: i32,
    pub challenge_time: i32,
    pub system_instruction: String,
    pub greeting_text: String,
    pub start_text: String,
    pub rating_threshold: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sponsor {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub background_url: String,
    pub private_key: String,
    pub token_mint: String,
    pub original_tokens: i32,
    pub available_tokens: i32,
    pub reward_tokens: i32,
    pub challenge_time: i32,
    pub system_instruction: String,
    pub greeting_text: String,
    pub start_text: String,
    pub end_text: String,
    pub won_text: String,
    pub lost_text: String,
    pub rating_threshold: i32,
}

pub async fn launchpad(
    Json(new_sponsor): Json<LaunchpadArgs>,
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

    Json(sponsor).into_response()
}
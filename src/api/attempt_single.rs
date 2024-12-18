use axum::response::IntoResponse;
use axum::extract::Path;
use axum::Json;
use reqwest::StatusCode;

use crate::api::Attempt;


pub async fn attempt_single(
    Path(id): Path<String>,
) -> impl IntoResponse {


    let attempt = Attempt {
        id,
        phone_number: "1234567890".to_string(),
        created_at: 1734529222,
        updated_at: 1734579250,
        video_url: "https://osco.digital/why-fun-crab-video.mp4".to_string(),
        twitter_url: "https://x.com/theCRABsite/status/1869299110026899949".to_string(),
        is_winner: false,
        sponsor_question: "Why should I give you 1000 dollars worth of CRAB tokens?".to_string(),
        sponsor_name: "CRAB".to_string(),
        sponsor_token_mint: "Gst1mHM4sqEgUeZVNLRaDhVZBKngsnpch4abiVyGpump".to_string(),
        sponsor_total_reward: 100000,
        sponsor_attempt_reward: 1000,
        sponsor_background_url: "https://osco.digital/why-fun.jpg".to_string(),
        sponsor_challenge_time: 30,
        challenge_transcript: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.".to_string(),
        challenge_status: "ongoing".to_string(),
    };

    return Json(attempt).into_response();
}
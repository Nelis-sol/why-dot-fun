use axum::response::IntoResponse;
use axum::Json;
use serde::{Serialize, Deserialize};
use crate::api::Attempt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AttemptList {
    pub attempts: Vec<Attempt>,
}


pub async fn attempt_list() -> impl IntoResponse {

    let mut attempt_list = AttemptList {
        attempts: vec![],
    };

    let attempt1 = Attempt {
        id: "EG3VTogv4sKch4NtgVMCUr3NXwLsauzKXtQ2ffhEiC3h".to_string(),
        phone_number: "1234567890".to_string(),
        created_at: Some(1734529222),
        updated_at: Some(1734579250),
        video_url: Some("https://osco.digital/why-fun-crab-video.mp4".to_string()),
        twitter_url: Some("https://x.com/theCRABsite/status/1869299110026899949".to_string()),
        is_winner: Some(false),
        sponsor_question: Some("Why should I give you 1000 dollars worth of CRAB tokens?".to_string()),
        sponsor_name: Some("CRAB".to_string()),
        sponsor_token_mint: Some("Gst1mHM4sqEgUeZVNLRaDhVZBKngsnpch4abiVyGpump".to_string()),
        sponsor_total_reward: Some(100000),
        sponsor_attempt_reward: Some(1000),
        sponsor_background_url: Some("https://osco.digital/why-fun.jpg".to_string()),
        sponsor_challenge_time: Some(30),
        challenge_transcript: Some("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.".to_string()),
        challenge_status: Some("ongoing".to_string()),
    };

    let attempt2 = Attempt {
        id: "6rwY2CMnzSbfonKzUiDE9EkeZbMF8hT6vMQ5uCrKNmuj2".to_string(),
        phone_number: "1234567890".to_string(),
        created_at: Some(1734529222),
        updated_at: Some(1734579250),
        video_url: Some("https://osco.digital/why-fun-crab-video.mp4".to_string()),
        twitter_url: Some("https://x.com/theCRABsite/status/1869299110026899949".to_string()),
        is_winner: Some(false),
        sponsor_question: Some("Why should I give you 1000 dollars worth of CRAB tokens?".to_string()),
        sponsor_name: Some("CRAB".to_string()),
        sponsor_token_mint: Some("Gst1mHM4sqEgUeZVNLRaDhVZBKngsnpch4abiVyGpump".to_string()),
        sponsor_total_reward: Some(100000),
        sponsor_attempt_reward: Some(1000),
        sponsor_background_url: Some("https://osco.digital/why-fun.jpg".to_string()),
        sponsor_challenge_time: Some(30),
        challenge_transcript: Some("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.".to_string()),
        challenge_status: Some("ongoing".to_string()),
    };

    attempt_list.attempts.push(attempt1);   
    attempt_list.attempts.push(attempt2);

    return Json(attempt_list).into_response();
}
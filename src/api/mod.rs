pub mod attempt_list;
pub mod attempt_single;
pub mod launchpad;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attempt {
    // id of the attempt
    pub id: String,
    // phone number of the user
    pub phone_number: String,
    // attempt created at
    pub created_at: Option<i32>,
    // attempt updated at
    pub updated_at: Option<i32>,
    // video url of attempt
    pub video_url: Option<String>,
    // twitter url of attempt
    pub twitter_url: Option<String>,
    // is the attempt a winner
    pub is_winner: Option<bool>,
    // sponsored question / challenge
    pub sponsor_question: Option<String>,
    // name of the sponsor
    pub sponsor_name: Option<String>,
    // sponsored token mint
    pub sponsor_token_mint: Option<String>,
    // sponsored total reward
    pub sponsor_total_reward: Option<i32>,
    // sponsored reward per attempt
    pub sponsor_attempt_reward: Option<i32>,
    // background url of the sponsor image or video
    pub sponsor_background_url: Option<String>,
    // time user has to complete the challenge
    pub sponsor_challenge_time: Option<i32>,
    // transcript of the challenge
    pub challenge_transcript: Option<String>,
    // status of the challenge
    pub challenge_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SponsorArgs {
    pub name: String,
    pub background_url: String,
    pub token_mint: String,
    pub original_tokens: i32,
    pub available_tokens: i32,
    pub reward_tokens: i32,
    pub challenge_time: i32,
    pub system_instruction: String,
    pub start_text: String,
    pub rating_threshold: i32,
}
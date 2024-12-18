pub mod attempt_list;
pub mod attempt_single;
pub mod launchpad;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attempt {
    // id of the attempt
    pub id: String,
    // attempt created at
    pub created_at: u64,
    // attempt updated at
    pub updated_at: u64,
    // video url of attempt
    pub video_url: String,
    // twitter url of attempt
    pub twitter_url: String,
    // is the attempt a winner
    pub is_winner: bool,
    // sponsored question / challenge
    pub sponsor_question: String,
    // name of the sponsor
    pub sponsor_name: String,
    // sponsored token mint
    pub sponsor_token_mint: String,
    // sponsored total reward
    pub sponsor_total_reward: u64,
    // sponsored reward per attempt
    pub sponsor_attempt_reward: u64,
    // background url of the sponsor image or video
    pub sponsor_background_url: String,
    // time user has to complete the challenge
    pub sponsor_challenge_time: u16,
    // transcript of the challenge
    pub challenge_transcript: String,
    // status of the challenge
    pub challenge_status: String,
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
    pub greeting_text: String,
    pub start_text: String,
    pub rating_threshold: i32,
}
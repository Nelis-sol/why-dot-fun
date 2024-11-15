use crate::secrets::Secrets;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Connect to the database with the given secrets.
    pub async fn new(secrets: &Secrets) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&secrets.database_url)
            .await
            .expect("Failed to connect to the database");

        Self { pool }
    }

    /// Gets a random sponsor from the database that meets these requirements:
    /// - The sponsor is active
    /// - The sponsor has enough available tokens to reward the user
    pub async fn get_random_sponsor(&self) -> Result<Sponsor> {
        Ok(sqlx::query_as!(
            Sponsor,
            r#"
                SELECT * FROM sponsors
                WHERE active = true
                AND available_tokens >= reward_tokens
                ORDER BY RANDOM()
                LIMIT 1
            "#
        )
        .fetch_one(&self.pool)
        .await?)
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Sponsor {
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub background_url: String,
    pub token_address: String,
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
}

use crate::secrets::Secrets;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPoolOptions, PgPool};
use serde::{Serialize, Deserialize};
use crate::api::Attempt;

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

    /// Gets the sponsor with the given ID from the database.
    pub async fn get_sponsor_by_id(&self, id: i32) -> Result<Sponsor> {
        Ok(sqlx::query_as!(
            Sponsor,
            r#"
                SELECT * FROM sponsors
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    /// Creates a new winner in the database with the given name and sponsor ID.
    /// Uses a random UUID as the private key that the user can use to claim their reward.
    pub async fn create_winner(&self, name: String, sponsor_id: i32) -> Result<Winner> {
        Ok(sqlx::query_as!(
            Winner,
            r#"
                INSERT INTO winners (key, name, sponsor_id)
                VALUES (gen_random_uuid(), $1, $2)
                RETURNING *
            "#,
            name,
            sponsor_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    /// Gets the winner with the given key from the database.
    /// Returns `None` if there is no winner with the given key.
    pub async fn get_winner_by_key(&self, key: &str) -> Result<Option<Winner>> {
        Ok(sqlx::query_as!(
            Winner,
            r#"
                SELECT * FROM winners
                WHERE key = $1
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await?)
    }


    /// Gets the winner with the given key from the database.
    /// Returns `None` if there is no winner with the given key.
    pub async fn get_attempt_by_pubkey(&self, public_key: String) -> Result<Option<Attempt>> {
        Ok(sqlx::query_as!(
            Attempt,
            r#"
                SELECT * FROM attempts
                WHERE pubkey = $1
            "#,
            public_key
        )
        .fetch_optional(&self.pool)
        .await?)
    }


    /// Gets the winner with the given key from the database.
    /// Returns `None` if there is no winner with the given key.
    pub async fn get_all_attempts_last_14_days(&self) -> Result<Vec<Attempt>> {
        Ok(sqlx::query_as!(
            Attempt,
            r#"
                SELECT * FROM attempts
                WHERE created_at BETWEEN NOW() - INTERVAL '14 days' AND NOW()
                ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?)
    }


    /// Withdraws the reward tokens from the sponsor with the given ID.
    /// Returns an error if there was a communication error with the database.
    /// Returns `None` if the sponsor does not have enough available tokens to withdraw.
    /// Returns the amount of withdrawn tokens if the sponsor has enough available tokens.
    pub async fn withdraw_tokens(&self, sponsor_id: i32) -> Result<Option<WithdrawnTokens>> {
        Ok(sqlx::query_as!(
            WithdrawnTokens,
            r#"
                UPDATE sponsors
                SET available_tokens = available_tokens - reward_tokens
                WHERE id = $1
                AND available_tokens >= reward_tokens
                RETURNING reward_tokens AS amount
            "#,
            sponsor_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    /// Gets the user with the given phone number from the database. Creates a new user if the
    /// user does not yet exist in the database.
    pub async fn get_or_insert_user_by_phone_number(&self, phone_number: &str) -> Result<User> {
        Ok(sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (phone_number, attempts_today, last_attempt, banned)
                VALUES ($1, 1, now(), false)
                ON CONFLICT (phone_number) DO UPDATE
                SET attempts_today = users.attempts_today + 1
                RETURNING *
                
            "#,
            phone_number
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn update_user(&self, user: &User) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET attempts_today = $1, last_attempt = now(), banned = $2
                WHERE phone_number = $3
            "#,
            user.attempts_today,
            user.banned,
            user.phone_number
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

     /// Creates a new sponsor in the database.
    pub async fn create_sponsor(&self, sponsor: Sponsor) -> Result<Sponsor> {
        Ok(sqlx::query_as!(
            Sponsor,
            r#"
                INSERT INTO sponsors (
                name, 
                active, 
                background_url, 
                private_key,
                token_mint, 
                original_tokens, 
                available_tokens, 
                reward_tokens, 
                challenge_time,
                system_instruction,
                greeting_text, 
                start_text, 
                end_text,
                won_text,
                lost_text,
                rating_threshold
            )
                VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
                )
                RETURNING *
            "#,
            sponsor.name,
            sponsor.active,
            sponsor.background_url,
            sponsor.private_key,
            sponsor.token_mint,
            sponsor.original_tokens,
            sponsor.available_tokens,
            sponsor.reward_tokens,
            sponsor.challenge_time,
            sponsor.system_instruction,
            sponsor.greeting_text,
            sponsor.start_text,
            sponsor.end_text,
            sponsor.won_text,
            sponsor.lost_text,
            sponsor.rating_threshold
        )
        .fetch_one(&self.pool)
        .await?)
    }


    /// Creates a new attempt in the database.
    pub async fn create_attempt_with_sponsor(&self, user: &User, sponsor: &Sponsor) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO attempts (
                phone_number,
                sponsor_question,
                sponsor_name,
                sponsor_token_mint,
                sponsor_total_reward,
                sponsor_attempt_reward,
                sponsor_background_url,
                sponsor_challenge_time
            )
                VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8
                )
            "#,
            user.phone_number,
            sponsor.start_text,
            sponsor.name,
            sponsor.token_mint,
            sponsor.original_tokens,
            sponsor.reward_tokens,
            sponsor.background_url,
            sponsor.challenge_time,
        )
        .execute(&self.pool)
        .await?;

        Ok(())  
    }


    pub async fn update_attempt_video(&self, caller_phone_number: String, video_url: String) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE attempts
                SET video_url = $1
                WHERE phone_number = $2
            "#,
            video_url,
            caller_phone_number
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_attempt_winner(&self, caller_phone_number: String, is_winner: bool) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE attempts
                SET is_winner = $1
                WHERE phone_number = $2
            "#,
            is_winner,
            caller_phone_number
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }



}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct WithdrawnTokens {
    pub amount: i32,
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Winner {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub sponsor_id: i32,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct User {
    pub phone_number: String,
    pub attempts_today: i32,
    pub last_attempt: DateTime<Utc>,
    pub banned: bool,
}

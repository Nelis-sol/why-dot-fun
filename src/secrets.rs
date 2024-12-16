use std::env::var;

#[derive(Debug, Clone)]
pub struct Secrets {
    pub global_url: String,
    pub database_url: String,
    pub twilio_phone_number: String,
    pub twilio_account_id: String,
    pub twilio_auth_token: String,
    pub novita_api_key: String,
    pub review_token: String,
    pub twitter_api_key: String,
    pub twitter_api_secret: String,
    pub twitter_access_token: String,
    pub twitter_access_secret: String,
}

impl Secrets {
    pub fn from_env() -> Self {
        Self {
            global_url: var("GLOBAL_URL").expect("GLOBAL_URL must be set"),
            database_url: var("DATABASE_URL").expect("DATABASE_URL must be set"),
            twilio_phone_number: var("TWILIO_PHONE_NUMBER")
                .expect("TWILIO_PHONE_NUMBER must be set"),
            twilio_account_id: var("TWILIO_ACCOUNT_ID").expect("TWILIO_ACCOUNT_ID must be set"),
            twilio_auth_token: var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN must be set"),
            novita_api_key: var("NOVITA_API_KEY").expect("NOVITA_API_KEY must be set"),
            review_token: var("REVIEW_TOKEN").expect("REVIEW_TOKEN must be set"),
            twitter_api_key: var("TWITTER_API_KEY").expect("TWITTER_API_KEY must be set"),
            twitter_api_secret: var("TWITTER_API_SECRET").expect("TWITTER_API_SECRET must be set"),
            twitter_access_token: var("TWITTER_ACCESS_TOKEN")
                .expect("TWITTER_ACCESS_TOKEN must be set"),
            twitter_access_secret: var("TWITTER_ACCESS_SECRET")
                .expect("TWITTER_ACCESS_SECRET must be set"),
        }
    }
}

use async_openai::Client as OpenaiClient;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use cache::CachedCall;
use database::Database;
use reqwest::Client as ReqwestClient;
use reqwest::StatusCode;
use secrets::Secrets;
use static_toml::static_toml;
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::services::ServeDir;
use twilio::Client as TwilioClient;
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

static_toml! { static CONFIG = include_toml!("Config.toml"); }

mod api;
mod cache;
mod claim;
mod database;
mod game;
mod review;
mod secrets;
mod solana;
mod twilio_token;
mod video;

#[tokio::main]
async fn main() {
    // Intitialize environment and logger
    dotenv::dotenv().ok();
    env_logger::init();

    // Load the secrets
    let secrets = Secrets::from_env();

    // Initialize the database
    log::info!("Connecting to the database");
    let database = Database::new(&secrets).await;

    // Initialize the twilio client
    log::info!("Initializing the Twilio client");
    let twilio = TwilioClient::new(&secrets.twilio_account_sid, &secrets.twilio_auth_token);

    // Initialize the OpenAI client
    log::info!("Initializing the OpenAI client");
    let openai = OpenaiClient::new();

    // Initialize the twitter client
    log::info!("Initializing the Twitter client");
    let twitter_token = Oauth1aToken::new(
        &secrets.twitter_api_key,
        &secrets.twitter_api_secret,
        &secrets.twitter_access_token,
        &secrets.twitter_access_secret,
    );
    let twitter = TwitterApi::new(twitter_token);

    // Initialize the reqwest client
    log::info!("Initializing the Reqwest client");
    let reqwest = ReqwestClient::new();

    // Initialize the conversation cache, maps the call id to all messages
    log::info!("Initializing the conversation cache");
    let cache = Arc::new(Mutex::new(HashMap::<String, CachedCall>::new()));

    // Initialize the TCP listener
    log::info!(
        "Connecting to the server at {}",
        CONFIG.settings.local_address
    );
    let tcp = TcpListener::bind(CONFIG.settings.local_address)
        .await
        .expect("Failed to connect to the server");

    // Initialize the webserver routes
    log::info!("Initializing the webserver routes");
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/start", post(game::start::start_handler))
        .route("/name", post(game::name::name_handler))
        .route("/challenge/start", post(game::challenge::start_handler))
        .route("/challenge/respond", post(game::challenge::respond_handler))
        .route("/end", post(game::end::end_handler))
        .route("/judge", post(game::judge::judge_handler))
        .route("/recording", post(game::recording::recording_handler))
        .route("/twilio-token", get(twilio_token::generate_jwt))
        .route(
            "/api/attempts/:id",
            get(api::attempt_single::attempt_single),
        )
        .route("/api/attempts", get(api::attempt_list::attempt_list))
        .route("/api/launchpad", post(api::launchpad::launchpad))
        .route(
            "/redirect-gather/*path",
            post(game::gather::redirect_gather_handler),
        )
        .nest_service("/claim", claim::router())
        .nest_service("/review", review::router())
        .nest_service("/static", ServeDir::new("static"))
        .layer(Extension(secrets))
        .layer(Extension(twilio))
        .layer(Extension(openai))
        .layer(Extension(twitter))
        .layer(Extension(reqwest))
        .layer(Extension(database))
        .layer(Extension(cache));

    // Start the webserver
    log::info!("Starting the webserver");
    axum::serve(tcp, router.into_make_service())
        .await
        .expect("Failed to start the server");
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// pub async fn attempts_all() -> impl IntoResponse {
//     StatusCode::OK
// }

// pub async fn attempts_by_id(Path(id): Path<String>) -> impl IntoResponse {
//     StatusCode::OK
// }

// pub async fn launchpad_create(
//     Json(payload): Json<LaunchpadCreatePayload>
// ) -> impl IntoResponse {
//     StatusCode::OK
// }

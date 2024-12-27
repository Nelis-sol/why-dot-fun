use axum::{routing::get, Router};
use tower_http::services::ServeFile;

mod check;
mod token;

pub fn router() -> Router {
    Router::new()
        .nest_service("/", ServeFile::new("static/call.html"))
        .route("/twilio-token", get(token::generate_jwt))
}

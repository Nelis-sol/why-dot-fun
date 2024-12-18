use axum::response::IntoResponse;
use reqwest::StatusCode;

pub async fn launchpad() -> impl IntoResponse {

    StatusCode::OK
}
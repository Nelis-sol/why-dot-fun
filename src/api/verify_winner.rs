use axum::response::IntoResponse;
use axum::Json;
use axum::Extension;
use crate::Database;
use serde::Serialize;
use crate::StatusCode;


#[derive(Serialize)]
struct WinnerResponse {
    is_winner: bool,
    winner_url: String,
}


pub async fn verify_winner(
    Extension(database): Extension<Database>,
    Json(phone_number): Json<String>,
) -> impl IntoResponse {

    let result = database
        .get_attempt_result_by_phone_number(phone_number)
        .await
        .unwrap();

    let is_winner = result.clone().unwrap().is_winner.unwrap();
    let winner_url = result.clone().unwrap().winner_url;

    let response = WinnerResponse {
        is_winner,
        winner_url,
    };

    (StatusCode::OK, Json(response)).into_response()
}
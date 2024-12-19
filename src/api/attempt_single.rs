use axum::response::IntoResponse;
use axum::extract::Path;
use axum::Json;
use chrono::Utc;
use crate::api::Attempt;
use axum::Extension;
use crate::Database;


pub async fn attempt_single(
    Extension(database): Extension<Database>,
    Path(public_key): Path<String>,
) -> impl IntoResponse {

    let attempt: Option<Attempt> = database
        .get_attempt_by_pubkey(public_key)
        .await
        .unwrap_or(None);

    return Json(attempt).into_response();
}
use axum::response::IntoResponse;
use axum::Json;
use crate::api::Attempt;
use axum::Extension;
use crate::Database;


pub async fn attempt_list(
    Extension(database): Extension<Database>,
) -> impl IntoResponse {

    let attempt_list: Vec<Attempt> = database
        .get_all_attempts_last_14_days()
        .await
        .unwrap_or(vec![]);

    return Json(attempt_list).into_response();
}
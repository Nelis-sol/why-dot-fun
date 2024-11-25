use crate::secrets::Secrets;
use anyhow::{anyhow, Context, Result};
use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::CookieJar;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

mod twitter;

pub fn router() -> Router {
    Router::new()
        .route("/", get(render_draft))
        .route("/approve", post(approve_draft))
        .route("/reject", post(reject_draft))
        .nest_service("/drafts", ServeDir::new("cache/drafts"))
        .layer(middleware::from_fn(check_token))
}

async fn render_draft() -> Result<DraftTemplate, StatusCode> {
    Ok(DraftTemplate {
        draft: get_next_draft().await.map_err(|e| {
            log::error!("Failed to get next draft: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?,
    })
}

async fn get_next_draft() -> Result<Option<Draft>> {
    let mut dir = tokio::fs::read_dir("cache/drafts")
        .await
        .context("Failed to read drafts directory")?;

    let entry = dir
        .next_entry()
        .await
        .context("Failed to read next draft directory entry")?;

    let entry = match entry {
        Some(entry) => entry,
        None => return Ok(None),
    };

    let call_sid = entry
        .file_name()
        .into_string()
        .map_err(|_| anyhow!("Failed to convert file name to string"))?
        .trim_end_matches(".mp4")
        .to_string();

    let comment = tokio::fs::read_to_string(format!("cache/recordings/{call_sid}/comment.txt"))
        .await
        .context("Failed to read comment file")?;

    Ok(Some(Draft { call_sid, comment }))
}

async fn approve_draft(
    twitter: Extension<TwitterApi<Oauth1aToken>>,
    reqwest: Extension<ReqwestClient>,
    secrets: Extension<Secrets>,
    draft: Json<Draft>,
) {
    log::debug!("Approving draft {}", draft.call_sid);
    let media_id = match twitter::upload_video(&reqwest, &secrets, &draft).await {
        Ok(media_id) => media_id,
        Err(e) => {
            log::error!("Failed to upload video: {:?}", e);
            return;
        }
    };

    log::debug!("Posting tweet with media_id {}", media_id);
    if let Err(e) = twitter::post_tweet(&twitter, media_id, &draft).await {
        log::error!("Failed to post tweet: {:?}", e);
        return;
    }

    log::debug!("Tweet posted successfully");
    let _ = tokio::fs::remove_file(format!("cache/drafts/{}.mp4", draft.call_sid)).await;
    let _ = tokio::fs::remove_dir_all(format!("cache/recordings/{}", draft.call_sid)).await;
}

async fn reject_draft(draft: Json<Draft>) {
    log::debug!("Rejecting draft {}", draft.call_sid);
    let _ = tokio::fs::remove_file(format!("cache/drafts/{}.mp4", draft.call_sid)).await;
        let _ = tokio::fs::remove_dir_all(format!("cache/recordings/{}", draft.call_sid)).await;
}

async fn check_token(
    secrets: Extension<Secrets>,
    cookies: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = cookies.get("review_token").map(|c| c.value());
    if token != Some(&secrets.review_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

#[derive(Debug, Deserialize, Serialize, Template)]
#[template(path = "review.html")]
pub struct DraftTemplate {
    draft: Option<Draft>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Draft {
    call_sid: String,
    comment: String,
}

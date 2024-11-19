use axum::{
    middleware::{self},
    routing::get,
    Router,
};

mod page;
mod verify;

pub fn router() -> Router {
    Router::new()
        .route("/", get(page::page_handler))
        .layer(middleware::from_fn(verify::verify))
}

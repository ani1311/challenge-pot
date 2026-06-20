use std::sync::Arc;

use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    routing::get,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    infrastructure::fs_persistence::FsPersistence,
    presentation::http::handlers::{get_leaderboard, health},
};

pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub persistence: Arc<FsPersistence>,
}

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/health", get(health))
        .route("/leaderboard", get(get_leaderboard))
        .with_state(state)
        .layer(cors)
}

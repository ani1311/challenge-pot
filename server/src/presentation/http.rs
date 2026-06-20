use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{infrastructure::fs_persistence::FsPersistence, presentation::http::handlers::{get_leaderboard, health}};

pub mod handlers;


#[derive(Clone)]
pub struct AppState {
    pub persistence: Arc<FsPersistence>
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/get_leaderboard", get(get_leaderboard))
        .with_state(state)

}

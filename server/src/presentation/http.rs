use std::sync::Arc;

use axum::{
    Router, http::{Method, header::CONTENT_TYPE}, middleware, routing::{get, post}
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    infrastructure::fs_persistence::FsPersistence,
    presentation::http::handlers::{get_leaderboard, health, track::track},
};

pub mod handlers;
mod auth;
mod error;

#[derive(Clone)]
pub struct AppState {
    pub persistence: Arc<FsPersistence>,
}

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let protected_routes = Router::new()
        .route("/leaderboard", get(get_leaderboard))
        .route("/track", post(track))
        .route_layer(middleware::from_fn(auth::auth_middleware));


    Router::new()
        .route("/health", get(health))
        .route("/login", post(handlers::login))
        .merge(protected_routes)
        .with_state(state)
        .layer(cors)
}

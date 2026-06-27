use std::{sync::Arc, time::Instant};

use axum::{
    Router,
    body::{Body, Bytes, to_bytes},
    extract::Request,
    http::{
        Method, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    infrastructure::fs_persistence::FsPersistence,
    presentation::http::handlers::{get_leaderboard, health, track::track},
};

mod auth;
mod error;
pub mod handlers;
mod mapper;

#[derive(Clone)]
pub struct AppState {
    pub persistence: Arc<FsPersistence>,
}

const LOG_BODY_LIMIT: usize = 64 * 1024;

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

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
        .layer(middleware::from_fn(log_request_response))
}

async fn log_request_response(request: Request, next: Next) -> Response {
    let started_at = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let request_headers = request.headers().clone();

    let (request_parts, request_body) = request.into_parts();
    let request_body = match to_bytes(request_body, LOG_BODY_LIMIT).await {
        Ok(body) => body,
        Err(error) => {
            tracing::warn!(
                method = %method,
                uri = %uri,
                error = %error,
                "failed to read request body",
            );

            return (StatusCode::BAD_REQUEST, "failed to read request body").into_response();
        }
    };

    tracing::info!(
        method = %method,
        uri = %uri,
        headers = ?request_headers,
        body = %body_to_log(&request_body),
        "request",
    );

    let request = Request::from_parts(request_parts, Body::from(request_body));
    let response = next.run(request).await;

    let status = response.status();
    let response_headers = response.headers().clone();
    let (response_parts, response_body) = response.into_parts();
    let response_body = match to_bytes(response_body, LOG_BODY_LIMIT).await {
        Ok(body) => body,
        Err(error) => {
            tracing::warn!(
                method = %method,
                uri = %uri,
                status = %status,
                error = %error,
                "failed to read response body",
            );

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to read response body",
            )
                .into_response();
        }
    };

    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status,
        elapsed_ms = started_at.elapsed().as_millis(),
        headers = ?response_headers,
        body = %body_to_log(&response_body),
        "response",
    );

    Response::from_parts(response_parts, Body::from(response_body))
}

fn body_to_log(body: &Bytes) -> String {
    String::from_utf8_lossy(body).into_owned()
}

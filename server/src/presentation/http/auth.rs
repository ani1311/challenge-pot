use axum::{body::Body, http::{Request, Response, StatusCode}, middleware::Next};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn auth_middleware(
    request: Request<Body>,
    next: Next) -> Result<Response<Body>, StatusCode>{
        Ok(next.run(request).await)
}
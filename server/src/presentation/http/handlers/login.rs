use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use common::login::{LoginRequest, LoginResponse};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    application,
    presentation::http::{AppState, auth::Claims, error::ApiError},
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let user = application::get_user(request.username, state.persistence.as_ref())
        .map_err(ApiError::user_lookup)?;

    // Utc
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.user_id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("some-secret".as_bytes()),
    )
    .expect("failed to create jwt");
    Ok(Json::from(LoginResponse { jwt: token }))
}

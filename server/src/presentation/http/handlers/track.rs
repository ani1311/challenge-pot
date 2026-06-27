use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use common::TrackRequest;

use crate::{
    application,
    domain::Activity,
    presentation::http::{AppState, auth::AuthUser, error::ApiError},
};

pub async fn track(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<TrackRequest>,
) -> Result<StatusCode, ApiError> {
    let activity = Activity::from(request.kind);

    application::track_activity(auth_user.user_id, activity, state.persistence.as_ref())
        .map_err(ApiError::internal)?;

    Ok(StatusCode::NO_CONTENT)
}

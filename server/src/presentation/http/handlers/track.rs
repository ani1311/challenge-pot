use axum::{Json, extract::State};
use common::{TrackRequest, api_error::ApiErrorResponse};

use crate::presentation::http::{AppState, error::ApiError};


pub async fn track(
    State(state): State<AppState>
) -> Result<(), ApiError> {
    Ok(())
}
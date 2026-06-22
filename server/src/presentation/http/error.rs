use std::io;

use axum::{Json, http::StatusCode, response::IntoResponse};
use common::api_error::{ApiErrorResponse, ErrorCode::{InternalError, UserNotFound}};

pub struct ApiError {
    status: StatusCode,
    body: ApiErrorResponse
}

impl ApiError {
    pub fn user_lookup(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => Self {
                status: StatusCode::NOT_FOUND,
                body: ApiErrorResponse {
                    code: UserNotFound,
                    message: "user_not_found".to_owned(),
                }
            },
            _ => Self::internal(error)
        }
    }

    pub fn internal(error: impl std::fmt::Display) -> Self {
        Self { 
            status:StatusCode::INTERNAL_SERVER_ERROR, 
            body: ApiErrorResponse{
                code: InternalError,
                message: error.to_string().to_owned(),
            } 
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self.body)).into_response()
    }
}
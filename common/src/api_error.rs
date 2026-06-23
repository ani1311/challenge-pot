use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    UserNotFound,
    InternalError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub code: ErrorCode,
    pub message: String,
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub jwt: String,
}

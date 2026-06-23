use common::{
    TrackRequest,
    api_error::{
        ApiErrorResponse,
        ErrorCode::{InternalError, UserNotFound},
    },
};
use gloo_net::http::Request;

use crate::api::client::api_url;

pub async fn track(token: String, request: TrackRequest) -> Result<(), String> {
    let response = Request::post(&api_url("/track"))
        .header("Authorization", &format!("Bearer {token}"))
        .json(&request)
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if response.ok() {
        return Ok(());
    }

    let error = response
        .json::<ApiErrorResponse>()
        .await
        .map_err(|_| "Could not save the activity. Try again.".to_owned())?;

    match error.code {
        UserNotFound => Err("Your user account no longer exists.".to_owned()),
        InternalError => Err("Could not save the activity. Try again.".to_owned()),
    }
}

use common::{api_error::{ApiErrorResponse, ErrorCode::{InternalError, UserNotFound}}, login::{LoginRequest, LoginResponse}};
use gloo_net::http::Request;

use crate::api::client::api_url;

pub async fn login(username: String) -> Result<LoginResponse, String> {
    let response = Request::post(&api_url("/login"))
        .json(&LoginRequest{
            username: username
        })
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if response.ok() {
        return response
            .json::<LoginResponse>()
            .await
            .map_err(|error| error.to_string());
    }

    let error = response
        .json::<ApiErrorResponse>()
        .await
        .map_err(|error| error.to_string())?;

    match error.code {
        UserNotFound => Err("User does not exist".to_owned()),
        InternalError => Err("Something went wrong".to_owned())
        
    }
}
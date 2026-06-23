use common::LeaderboardResponse;
use gloo_net::http::Request;

use crate::api::client::api_url;

pub async fn fetch_leaderboard(token: String) -> Result<LeaderboardResponse, String> {
    let resp = Request::get(&api_url("/leaderboard"))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    resp.json::<LeaderboardResponse>()
        .await
        .map_err(|err| err.to_string())
}

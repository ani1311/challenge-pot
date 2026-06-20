use common::LeaderboardResponse;
use gloo_net::http::Request;

use crate::api::client::api_url;

pub async fn fetch_leaderboard() -> Result<LeaderboardResponse, gloo_net::Error> {
    Request::get(&api_url("/v1/leaderboard"))
    .send()
    .await?
    .json::<LeaderboardResponse>()
    .await
}
use axum::{Json, extract::State};
use common::{ LeaderboardResponse};

use crate::{application::{self}, presentation::{http::AppState, mapper} };


pub async fn get_leaderboard(
    State(state): State<AppState>
) ->  Json<LeaderboardResponse>{
    
    let leaderboard = application::get_leaderboard(
        state.persistence.as_ref(),
        state.persistence.as_ref() );
    Json::from(mapper::to_leaderboard_response(leaderboard))
}

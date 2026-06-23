use common::{LeaderboardEntry, LeaderboardResponse, LeaderboardUser};

use crate::application::use_cases::get_leaderboard::Leaderboard;

pub fn to_leaderboard_response(leaderbard: Leaderboard) -> LeaderboardResponse {
    let entries: Vec<LeaderboardEntry> = leaderbard
        .rows
        .iter()
        .map(|v| LeaderboardEntry {
            user: LeaderboardUser {
                id: v.user.user_id.clone(),
                username: v.user.username.clone(),
            },
            points: v.points,
            rank: v.rank.clone(),
        })
        .collect();

    LeaderboardResponse { entries: entries }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LeaderboardEntry {
    pub user: LeaderboardUser,
    pub points: f32,
    pub rank: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LeaderboardUser {
    pub id: String,
    pub username: String,
}

use std::collections::HashMap;

use crate::{
    application::ports::{ActivityRepository, UserRepository},
    domain::{
        activity::Points,
        user::{User, UserId},
    },
};

pub struct LeaderboardRow {
    pub user: User,
    pub points: Points,
    pub rank: u32,
}

pub struct Leaderboard {
    pub rows: Vec<LeaderboardRow>,
}

pub fn get_leaderboard(
    activity_repo: &impl ActivityRepository,
    user_repo: &impl UserRepository,
) -> Leaderboard {
    let all_activity = activity_repo.get_all_activity_logs();

    let mut user_to_points: HashMap<UserId, Points> = HashMap::new();

    all_activity.iter().for_each(|a| {
        let points = user_to_points.entry(a.user.user_id.clone()).or_insert(0.0);
        *points += a.activity.points();
    });

    let mut rows: Vec<LeaderboardRow> = user_to_points
        .iter()
        .map(|(k, v)| {
            let user = user_repo.get_user_by_user_id(&k);

            LeaderboardRow {
                user: user,
                points: v.clone(),
                rank: 0,
            }
        })
        .collect();

    rows.sort_by(|a, b| {
        b.points
            .partial_cmp(&a.points)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for (index, row) in rows.iter_mut().enumerate() {
        row.rank = (index + 1) as u32
    }

    Leaderboard { rows }
}

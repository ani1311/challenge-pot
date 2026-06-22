use crate::{application::ports::ActivityRepository, domain::{Activity, ActivityLog, activity, user::{User, UserId}}};


pub fn track_activity(user_id: UserId, activity_log: ActivityLog, activity_repo: &impl ActivityRepository) -> Result<(), String> {
    todo!()
}
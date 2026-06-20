use crate::{domain::{Activity, ActivityLog, user::User}, infrastructure::fs_persistence::models::{StoredActivity::{self}, StoredActivityLog, StoredUser}};

pub fn to_user(stored_user: StoredUser) -> User {
    User { username: stored_user.username, user_id: stored_user.user_id }
}

pub fn to_activity(stored_activity: StoredActivity) -> Activity {
    match stored_activity {
            StoredActivity::EatSugar { grams } => { 
                Activity::EatSugar { grams }
            }
        }
}

pub fn to_activity_log(stored_activity_log: StoredActivityLog, user: User) -> ActivityLog {
    ActivityLog { 
        user: user, 
        activity: to_activity(stored_activity_log.activity), 
    }
}




use std::io::Error;

use crate::domain::{
    Activity, ActivityLog,
    user::{User, UserId},
};

pub trait UserRepository {
    fn get_user_by_user_id(&self, user_id: &UserId) -> User;
    fn get_user_by_username(&self, username: String) -> Result<User, Error>;
}

pub trait ActivityRepository {
    fn get_all_activity_logs(&self) -> Vec<ActivityLog>;
    fn save_activity(&self, user_id: &UserId, activity: Activity) -> Result<(), Error>;
}

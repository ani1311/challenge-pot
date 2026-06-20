use crate::domain::{ActivityLog, user::{User, UserId}};

pub trait UserRepository {
    fn get_user(&self, user_id: &UserId) -> User;
}

pub trait ActivityRepository {
    fn get_all_activity_logs(&self) -> Vec<ActivityLog>;
}


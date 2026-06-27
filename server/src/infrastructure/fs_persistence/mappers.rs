use crate::{
    domain::{Activity, ActivityLog, user::User}, infrastructure::fs_persistence::models::{
        StoredActivity, StoredActivityLog, StoredUser,
    },
};

pub fn to_user(stored_user: StoredUser) -> User {
    User {
        username: stored_user.username,
        user_id: stored_user.user_id,
    }
}

impl From<Activity> for StoredActivity {
    fn from(activity: Activity) -> Self {
        match activity {
            Activity::EatSugar { grams } => StoredActivity::EatSugar { grams },

            Activity::Walk { kilometers } => StoredActivity::Walk { kilometers },
            Activity::Hike { kilometers } => StoredActivity::Hike { kilometers },
            Activity::Run { kilometers } => StoredActivity::Run { kilometers },
            Activity::Swimming { kilometers } => StoredActivity::Swimming { kilometers },
            Activity::Bike { kilometers } => StoredActivity::Bike { kilometers },
            Activity::MountainBike { kilometers } => StoredActivity::MountainBike { kilometers },

            Activity::RaquetSport { hours } => StoredActivity::RaquetSport { hours },
        }
    }
}

impl From<StoredActivity> for Activity {
    fn from(stored_activity: StoredActivity) -> Self {
        match stored_activity {
            StoredActivity::EatSugar { grams } => Activity::EatSugar { grams },

            StoredActivity::Walk { kilometers } => Activity::Walk { kilometers },
            StoredActivity::Hike { kilometers } => Activity::Hike { kilometers },
            StoredActivity::Run { kilometers } => Activity::Run { kilometers },
            StoredActivity::Swimming { kilometers } => Activity::Swimming { kilometers },
            StoredActivity::Bike { kilometers } => Activity::Bike { kilometers },
            StoredActivity::MountainBike { kilometers } => Activity::MountainBike { kilometers },

            StoredActivity::RaquetSport { hours } => Activity::RaquetSport { hours },
        }
    }
}

pub fn to_activity_log(stored_activity_log: StoredActivityLog, user: User) -> ActivityLog {
    ActivityLog {
        user: user,
        activity: Activity::from(stored_activity_log.activity),
    }
}

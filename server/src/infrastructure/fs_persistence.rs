use std::{
    fs,
    path::PathBuf
};

use crate::{
    application::ports::{ActivityRepository, UserRepository}, domain::{Activity, user::UserId}, infrastructure::fs_persistence::{
        mappers::{to_activity_log, to_user}, models::{StoredActivity, StoredActivityLog},
    },
};

mod mappers;
mod models;

pub struct FsPersistence {
    data_dir: PathBuf,
}

impl FsPersistence {
    pub fn new(data_dir: PathBuf) -> Self {
        FsPersistence {
            data_dir,
        }
    }
}

const ACTIVITY_SUB_DIR: &str = "activity_log";

impl ActivityRepository for FsPersistence {
    fn get_all_activity_logs(&self) -> Vec<crate::domain::ActivityLog> {
        let mut logs = Vec::new();

        let activity_path = self.data_dir.join(ACTIVITY_SUB_DIR);
        if !activity_path.exists() {
            return logs;
        }

        for entry in fs::read_dir(activity_path).expect("failed to read dir") {
            let entry = entry.expect("failed to get activity");
            let path = entry.path();

            let is_json = path.extension().and_then(|ext| ext.to_str()) == Some("json");
            if !is_json {
                continue;
            }

            let content = fs::read_to_string(path).expect("failed to read activity_log");
            let stored_log = serde_json::from_str::<StoredActivityLog>(&content.as_str())
                .expect("failed to parse activity log");

            let user = self.get_user_by_user_id(&stored_log.user_id);
            let log = to_activity_log(stored_log, user);

            logs.push(log);
        }

        logs
    }

    fn save_activity(&self, user_id: &UserId, activity: Activity) -> Result<(), std::io::Error> {
        let activity_path = self.data_dir.join(ACTIVITY_SUB_DIR);
        fs::create_dir_all(&activity_path)?;

        let stored_activity = StoredActivity::from(activity); 
        let stored_log = StoredActivityLog {
            user_id: user_id.clone(),
            activity: stored_activity,
            created_at: chrono::Utc::now(),
        };
        let content = serde_json::to_string(&stored_log).map_err(std::io::Error::other)?;
        let id = uuid::Uuid::new_v4();

        let path = activity_path.join(format!("{id}.json"));

        fs::write(path, content)
    }
}

const USERS_DIR: &str = "users";

impl UserRepository for FsPersistence {
    fn get_user_by_user_id(
        &self,
        user_id: &crate::domain::user::UserId,
    ) -> crate::domain::user::User {
        let content = fs::read_to_string(
            self.data_dir
                .join(USERS_DIR)
                .join(user_id.to_owned() + ".json"),
        )
        .expect("failed to read user");

        let stored_user = serde_json::from_str(&content).expect("failed to parse user");
        to_user(stored_user)
    }

    fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<crate::domain::user::User, std::io::Error> {
        let users_path = self.data_dir.join(USERS_DIR);

        for entry in fs::read_dir(users_path).expect("failed to read users dir") {
            let entry = entry.expect("failed to get activity");
            let path = entry.path();

            let is_json = path.extension().and_then(|ext| ext.to_str()) == Some("json");
            if !is_json {
                continue;
            }

            let content = fs::read_to_string(path).expect("failed to read user");
            let user = to_user(serde_json::from_str(&content).expect("failed to parse user"));
            if user.username.eq_ignore_ascii_case(&username) {
                return Ok(user);
            }
        }

        Result::Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "user not found",
        ))
    }
}

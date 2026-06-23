use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoredUser {
    pub username: String,
    pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StoredActivity {
    EatSugar { grams: f32 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoredActivityLog {
    pub user_id: String,
    pub activity: StoredActivity,
}

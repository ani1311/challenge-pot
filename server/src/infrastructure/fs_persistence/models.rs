use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoredUser {
    pub username: String,
    pub user_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StoredActivity {
    EatSugar { grams: f32 },

    Walk { kilometers: f32 },
    Hike { kilometers: f32 },
    Run { kilometers: f32 },
    Swimming { kilometers: f32 },
    Bike { kilometers: f32 },
    MountainBike { kilometers: f32 },

    RaquetSport { hours: f32 },
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoredActivityLog {
    pub user_id: String,
    pub activity: StoredActivity,
    pub created_at: chrono::DateTime<Utc>,
}

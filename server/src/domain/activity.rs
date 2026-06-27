use crate::domain::user::User;

pub type Points = f32;

pub enum Activity {
    EatSugar { grams: f32 },

    Walk { kilometers: f32 },
    Hike{ kilometers: f32 },
    Run { kilometers: f32 },
    Swimming{ kilometers: f32 },
    Bike{ kilometers: f32 },
    MountainBike{ kilometers: f32 },

    RaquetSport { hours: f32 },
}

impl Activity {
    pub fn points(&self) -> Points {
        match self {
            Activity::EatSugar { grams } => {
                *grams
            },
            Activity::Walk { kilometers } => {
                -1.0 * (*kilometers) * 2.0
            },
            Activity::Hike{ kilometers } => {
                -1.0 * (*kilometers) * 3.0
            },
            Activity::Run{ kilometers } => {
                -1.0 * (*kilometers) * 4.0
            },
            Activity::Swimming{ kilometers } => {
                -1.0 * (*kilometers) * 4.0
            },
            Activity::Bike{ kilometers } => {
                -1.0 * (*kilometers) * 1.0
            },
            Activity::MountainBike{ kilometers } => {
                -1.0 * (*kilometers) * 5.0
            },
            Activity::RaquetSport{ hours} => {
                -1.0 * (*hours) * 5.0
            },
        }

    }
}

pub struct ActivityLog {
    pub user: User,
    pub activity: Activity,
}

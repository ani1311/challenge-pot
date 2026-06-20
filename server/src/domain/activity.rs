use crate::domain::{user::User};

pub type Points = f32;

pub enum Activity {
    EatSugar{grams: f32}
}


impl Activity {
    pub fn points(&self) -> Points {
        match self {
            Activity::EatSugar { grams } => { 
                *grams
            }
        }
    }
}

pub struct ActivityLog {
    pub user: User,
    pub activity: Activity
}
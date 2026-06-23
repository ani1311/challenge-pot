use std::io::Error;

use crate::{
    application::ports::ActivityRepository,
    domain::{Activity, user::UserId},
};

pub fn track_activity(
    user_id: UserId,
    activity: Activity,
    activity_repo: &impl ActivityRepository,
) -> Result<(), Error> {
    activity_repo.save_activity(&user_id, activity)
}

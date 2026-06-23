use std::io::Error;

use crate::{application::ports::UserRepository, domain::user::User};

pub fn get_user(username: String, user_repo: &impl UserRepository) -> Result<User, Error> {
    user_repo.get_user_by_username(username)
}

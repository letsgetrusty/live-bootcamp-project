use crate::domain::{email::Email, password::Password};

#[derive(Clone)]
pub struct User {
    pub requires_2fa: bool,
    pub password: Password,
    pub email: Email,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}

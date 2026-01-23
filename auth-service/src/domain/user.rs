pub struct User {
    pub requires_2fa: bool,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}

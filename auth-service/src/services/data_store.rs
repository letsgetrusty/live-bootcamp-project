use rand::Rng;
use crate::domain::{email::Email, password::Password, user::User};


#[derive(Debug, PartialEq,)]
pub enum UserStoreError {
    UserNotFound,
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}


#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError>;
}


#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    TokenAlreadyBanned,
    UnexpectedError,
}


#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn store_token(&self, token: &str) -> Result<(), BannedTokenStoreError>;
    async fn check_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}


#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}


#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}


#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        match uuid::Uuid::parse_str(&id) {
            Ok(_) => Ok(LoginAttemptId(id)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        LoginAttemptId(uuid::Uuid::new_v4().to_string())
    }
}

// TODO: Implement AsRef<str> for LoginAttemptId
impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.len() == 6 && code.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self(code))
        } else {
            Err("Invalid 2FA code".to_string())
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = rand::rng();
        let value: u32 = rng.random_range(0..1_000_000);
        Self(format!("{:06}", value))
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


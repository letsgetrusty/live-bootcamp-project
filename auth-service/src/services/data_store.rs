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

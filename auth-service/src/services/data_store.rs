use crate::domain::user::User;


#[derive(Debug, PartialEq,)]
pub enum UserStoreError {
    UserNotFound,
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}


#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError>;
}

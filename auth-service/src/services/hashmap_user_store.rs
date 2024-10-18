use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}
impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if let Some(_) = self.users.get(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(existing_user) = self.users.get(email) {
            return Ok(existing_user.clone());
        }
        Err(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password == password {
            return Ok(());
        }
        Err(UserStoreError::InvalidCredentials)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let email = "sample@domain.com".to_string();
        let password = "p@$$w0rd".to_string();
        let user = User::new(
            email.clone(),
            password.clone(),
            false,
        );
        let result = store.add_user(user.clone());
        assert!(result.is_ok());
        let new_user = store.users.get(&email);
        assert!(new_user.is_some());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = "sample@domain.com".to_string();
        let password = "p@$$w0rd".to_string();
        let user = User::new(
            email.clone(),
            password.clone(),
            false,
        );
        let _ = store.add_user(user.clone());

        let result = store.get_user(&email);
        assert!(result.is_ok());
        let result = store.get_user("wrong@domain.com");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = "sample@domain.com".to_string();
        let password = "p@$$w0rd".to_string();
        let user = User::new(
            email.clone(),
            password.clone(),
            false,
        );
        let _ = store.add_user(user.clone());
        let result = store.validate_user(&email, &password);
        assert!(result.is_ok());
        let result = store.validate_user(&email, "wrong password");
        assert!(result.is_err());
    }
}

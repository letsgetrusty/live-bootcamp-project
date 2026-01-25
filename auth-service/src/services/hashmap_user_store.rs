use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::user::User;
use super::data_store::{UserStore, UserStoreError};


#[derive(Default)]
pub struct HashmapUserStore {
    users: RwLock<HashMap<String, User>>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&self, user: User) -> Result<(), UserStoreError> {
        if self.users.read().await.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.write().await.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users.read().await.get(email).cloned().ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), true);
        assert!(store.add_user(user).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), true);
        store.add_user(user).await.unwrap();
        assert!(store.get_user("test@example.com").await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), true);
        store.add_user(user).await.unwrap();
        assert!(store.validate_user("test@example.com", "password").await.is_ok());
    }
}

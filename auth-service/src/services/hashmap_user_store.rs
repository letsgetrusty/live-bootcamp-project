use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::{email::Email, password::Password, user::User};
use super::data_store::{UserStore, UserStoreError};


#[derive(Default)]
pub struct HashmapUserStore {
    users: RwLock<HashMap<String, User>>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&self, user: User) -> Result<(), UserStoreError> {
        let mut guard  = self.users.write().await;

        if guard.contains_key(user.email.as_ref()) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        guard.insert(user.email.as_ref().to_string(), user);

        Ok(())
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        self.users.read().await.get(email.as_ref()).cloned().ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
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
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let user = User::new(email, password, true);
        assert!(store.add_user(user).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_user() {
        let store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let user = User::new(email.clone(), password, true);
        store.add_user(user).await.unwrap();
        assert!(store.get_user(email).await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let password = Password::parse("password".to_string()).unwrap();
        let user = User::new(email.clone(), password.clone(), true);
        store.add_user(user).await.unwrap();
        assert!(store.validate_user(email, password).await.is_ok());
    }
}

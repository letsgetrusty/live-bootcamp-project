use std::collections::HashMap;
use crate::domain::user::User;


#[derive(Debug, PartialEq,)]
pub enum UserStoreError {
    UserNotFound,
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}


#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}


impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), true);
        assert!(store.add_user(user).is_ok());
    }

    #[test]
    fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), true);
        store.add_user(user).unwrap();
        assert!(store.get_user("test@example.com").is_ok());
    }

    #[test]
    fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@example.com".to_string(), "password".to_string(), true);
        store.add_user(user).unwrap();
        assert!(store.validate_user("test@example.com", "password").is_ok());
    }
}

use std::collections::HashSet;

use tokio::sync::RwLock;

use crate::services::data_store::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: RwLock<HashSet<String>>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_token(&self, token: &str) -> Result<(), super::data_store::BannedTokenStoreError> {
        let mut guard = self.tokens.write().await;
        if guard.contains(token) {
            println!("Guard already contains token");
            return Err(BannedTokenStoreError::TokenAlreadyBanned);
        }
        guard.insert(token.to_string());
        Ok(())
    }

    async fn check_token(&self, token: &str) -> Result<bool, super::data_store::BannedTokenStoreError> {
        let res = self.tokens.read().await.contains(token);
        Ok(res)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task::{self, futures};

    #[tokio::test]
    async fn store_token_new_token_ok() {
        let store = HashsetBannedTokenStore::default();

        let res = store.store_token("abc").await;
        assert_eq!(res, Ok(()));
    }

    #[tokio::test]
    async fn store_token_same_token_twice_returns_token_already_banned() {
        let store = HashsetBannedTokenStore::default();

        assert_eq!(store.store_token("abc").await, Ok(()));
        assert_eq!(
            store.store_token("abc").await,
            Err(BannedTokenStoreError::TokenAlreadyBanned)
        );
    }

    #[tokio::test]
    async fn check_token_false_before_store_true_after_store() {
        let store = HashsetBannedTokenStore::default();

        assert_eq!(store.check_token("abc").await, Ok(false));
        assert_eq!(store.store_token("abc").await, Ok(()));
        assert_eq!(store.check_token("abc").await, Ok(true));
    }
}


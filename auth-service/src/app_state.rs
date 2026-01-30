use std::{ops::Deref, sync::Arc};
use tokio::sync::Mutex;

use crate::services::{data_store::{BannedTokenStore, TwoFACodeStore, UserStore}, hashmap_two_fa_code_store::HashmapTwoFACodeStore, hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashsetBannedTokenStore};


#[derive(Clone)]
pub struct UserStoreType(pub Arc<Box<dyn UserStore>>);

impl Deref for UserStoreType {
    type Target = Arc<Box<dyn UserStore>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UserStoreType {
    fn default() -> Self {
        let store: Box<dyn UserStore> = Box::new(HashmapUserStore::default());
        Self(Arc::new(store))
    }
}


#[derive(Clone)]
pub struct BannedTokenStoreType(pub Arc<Box<dyn BannedTokenStore>>);

impl Deref for BannedTokenStoreType {
    type Target = Arc<Box< dyn BannedTokenStore>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for BannedTokenStoreType {
    fn default() -> Self {
        let store: Box<dyn BannedTokenStore> = Box::new(HashsetBannedTokenStore::default());
        Self(Arc::new(store))
    }
}


#[derive(Clone)]
pub struct TwoFACodeStoreType(pub Arc<Mutex<Box<dyn TwoFACodeStore>>>);

impl Deref for TwoFACodeStoreType {
    type Target = Arc<Mutex<Box< dyn TwoFACodeStore>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for TwoFACodeStoreType {
    fn default() -> Self {
        let store: Mutex<Box<dyn TwoFACodeStore>> = Mutex::new(Box::new(HashmapTwoFACodeStore::default()));
        Self(Arc::new(store))
    }
}



#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType, // New!
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        banned_token_store: BannedTokenStoreType,
        two_fa_code_store: TwoFACodeStoreType, // New!
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_code_store, // New!
        }
    }

    // // Optional ergonomic constructor (useful for DI/tests)
    // pub fn with_store(store: impl UserStore + 'static) -> Self {
    //     let store: Box<dyn UserStore> = Box::new(store);
    //     Self {
    //         user_store: UserStoreType(Arc::new(RwLock::new(store))),
    //     }
    // }
}

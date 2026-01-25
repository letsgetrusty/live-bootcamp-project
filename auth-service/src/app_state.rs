use std::{ops::Deref, sync::Arc};
use crate::services::{data_store::UserStore, hashmap_user_store::HashmapUserStore};


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
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }

    // // Optional ergonomic constructor (useful for DI/tests)
    // pub fn with_store(store: impl UserStore + 'static) -> Self {
    //     let store: Box<dyn UserStore> = Box::new(store);
    //     Self {
    //         user_store: UserStoreType(Arc::new(RwLock::new(store))),
    //     }
    // }
}

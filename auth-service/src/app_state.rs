use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, UserStore};

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
}

impl AppState {
    pub fn new(banned_token_store: BannedTokenStoreType, user_store: UserStoreType) -> Self {
        Self {
            user_store,
            banned_token_store,
        }
    }
}

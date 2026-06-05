use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::domain::{AccountRepo, DomainError, SolanaAccount};

#[derive(Debug, Default)]
pub struct InMemoryAccountRepo {
    pub store: Arc<RwLock<HashMap<String, SolanaAccount>>>,
}

impl InMemoryAccountRepo {
    pub fn new() -> Self {
      Self::default()
    }
}

#[async_trait]
impl AccountRepo for InMemoryAccountRepo {
    async fn get(&self, key: &str) -> Result<SolanaAccount, DomainError> {
        let store = self.store.read().await;
        let account = store.get(key).cloned().ok_or(DomainError::NotFound)?;
        Ok(account)
    }

    async fn create(&self, account: SolanaAccount) -> Result<(), DomainError> {
        let mut store = self.store.write().await;
        if store.contains_key(&account.pubkey) {
            return Err(DomainError::Conflict(account.pubkey.to_string()));
        }
        store.insert(account.pubkey.clone(), account);
        Ok(())
    }

    async fn list(&self) -> Result<Vec<SolanaAccount>, DomainError> {
        let store = self.store.read().await;
        let accounts = store.values().cloned().collect::<Vec<SolanaAccount>>();
       Ok(accounts)
    }


}

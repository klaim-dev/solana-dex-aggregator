use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::{AccountRepo, DomainError, SolanaAccount};

#[derive(Debug)]
pub struct InMemoryAccountRepo {
    store: HashMap<String, SolanaAccount>,
}

impl InMemoryAccountRepo {
    pub fn new() -> Self {
        let key = "1".repeat(32);
            let account = SolanaAccount {
            pubkey: "1".repeat(12),
            owner: "2".repeat(12),
            lamports: 1_000_000,
        };
        let store = HashMap::from([(key.clone(), account.clone())]);
        Self {store}
    }
}

#[async_trait]
impl AccountRepo for InMemoryAccountRepo {
    async fn get(&self, key: &str) -> Result<SolanaAccount, DomainError> {
        self.store.get(key).cloned().ok_or(DomainError::NotFound)
    }
}

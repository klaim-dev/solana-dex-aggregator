use std::{str::FromStr, sync::Arc};

use solana_sdk::pubkey::Pubkey;

use crate::domain::{AccountRepo, DomainError, SolanaAccount};

pub struct AccountService {
    repo: Arc<dyn AccountRepo>,
}

impl AccountService {
    pub fn new(repo: Arc<dyn AccountRepo>) -> Self {
        Self { repo }
    }

    pub async fn get_account(&self, key: &str) -> Result<SolanaAccount, DomainError> {
        Self::validate_key(key)?;
        self.repo.get(key).await
    }

    pub async fn create_account(&self, account: SolanaAccount) -> Result<(), DomainError> {
        Self::validate_key(&account.pubkey)?;
        self.repo.create(account).await
    }

    pub async fn list(&self) -> Result<Vec<SolanaAccount>, DomainError> {
        self.repo.list().await
    }

    pub fn validate_key(key: &str) -> Result<(), DomainError> {
        Pubkey::from_str(key).map_err(|_| DomainError::InvalidPubkey(key.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tokio::sync::RwLock;

    use crate::infra::repo::in_memory::InMemoryAccountRepo;

    #[tokio::test]
    async fn returns_account_when_present() {
        let key = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string();
        let account = SolanaAccount {
            pubkey: key.clone(),
            owner: "11111111111111111111111111111111".to_string(),
            lamports: 1_000_000,
        };
        let store = Arc::new(RwLock::new(HashMap::from([(key.clone(), account.clone())])));
        let repo = InMemoryAccountRepo { store };
        let service = AccountService::new(Arc::new(repo));
        let result = service.get_account(&key).await;

        assert_eq!(result, Ok(account))
    }

    #[tokio::test]
    async fn returns_not_found_when_absent() {
        let repo = InMemoryAccountRepo::new();
        let service = AccountService::new(Arc::new(repo));

        let key = "11111111111111111111111111111111";
        let result = service.get_account(&key).await;

        assert_eq!(result, Err(DomainError::NotFound));
    }

    #[tokio::test]
    async fn returns_invalid_key_without_touching_repo() {
        let repo = InMemoryAccountRepo::new();
        let service = AccountService::new(Arc::new(repo));

        let key = "short";
        let result = service.get_account(key).await;

        assert_eq!(result, Err(DomainError::InvalidPubkey(key.to_string())));
    }
}

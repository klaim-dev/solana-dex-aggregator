use crate::domain::{AccountRepo, DomainError, SolanaAccount};

pub struct AccountService<R: AccountRepo> {
    repo: R,
}

impl<R: AccountRepo> AccountService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_account(&self, key: &str) -> Result<SolanaAccount, DomainError> {
        if key.len() < 32 || key.len() > 44 {
            return Err(DomainError::InvalidKey(key.to_string()));
        }
        self.repo.get(key).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::infra::repo::in_memory::InMemoryAccountRepo;

    #[tokio::test]
    async fn returns_account_when_present() {
        let key = "a".repeat(44);
        let account = SolanaAccount {
            pubkey: "1".repeat(12),
            owner: "2".repeat(12),
            lamports: 1_000_000,
        };
        let store = HashMap::from([(key.clone(), account.clone())]);
        let repo = InMemoryAccountRepo::from_store(store);
        let service = AccountService::new(repo);
        let result = service.get_account(&key).await;

        assert_eq!(result, Ok(account))
    }

    #[tokio::test]
    async fn returns_not_found_when_absent() {
        let repo = InMemoryAccountRepo::new();
        let service = AccountService::new(repo);

        let key = "b".repeat(44);
        let result = service.get_account(&key).await;

        assert_eq!(result, Err(DomainError::NotFound));
    }

    #[tokio::test]
    async fn returns_invalid_key_without_touching_repo() {
        let repo = InMemoryAccountRepo::new();
        let service = AccountService::new(repo);

        let key = "short";
        let result = service.get_account(key).await;

        assert_eq!(result, Err(DomainError::InvalidKey(key.to_string())));
    }
}

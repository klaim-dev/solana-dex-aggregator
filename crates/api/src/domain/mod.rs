pub mod error;
use async_trait::async_trait;
pub use error::DomainError;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct SolanaAccount {
    pub pubkey: String,
    pub owner: String,
    pub lamports: u64,
}

#[async_trait]
pub trait AccountRepo: Send + Sync {
    async fn get(&self, key: &str) -> Result<SolanaAccount, DomainError>;
}

#[async_trait]
impl AccountRepo for Arc<dyn AccountRepo> {
    async fn get(&self, key: &str) -> Result<SolanaAccount, DomainError> {
        (**self).get(key).await
    }
}

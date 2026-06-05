pub mod error;
use async_trait::async_trait;
pub use error::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct SolanaAccount {
    pub pubkey: String,
    pub owner: String,
    pub lamports: u64,
}

#[async_trait]
pub trait AccountRepo: Send + Sync {
    async fn get(&self, key: &str) -> Result<SolanaAccount, DomainError>;
    async fn create(&self, account: SolanaAccount) -> Result<(), DomainError>;
    async fn list(&self) -> Result<Vec<SolanaAccount>, DomainError>;
}
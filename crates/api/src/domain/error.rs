#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    #[error("not found")]
    NotFound,
    #[error("invalid key {0}")]
    InvalidKey(String),
    #[error("the account already exists: {0}")]
    Conflict(String),
    #[error("invalid account pubkey: {0}")]
    InvalidPubkey(String),
}

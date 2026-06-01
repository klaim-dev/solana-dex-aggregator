#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    #[error("not found")]
    NotFound,
    #[error("invalid key {0}")]
    InvalidKey(String),
}

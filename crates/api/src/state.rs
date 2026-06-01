use crate::config::Config;
use crate::domain::AccountRepo;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub account_repo: Arc<dyn AccountRepo>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("missing env var: {0}")]
    Missing(&'static str),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub solana_rpc_url: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    fn from_source(get: impl Fn(&str) -> Option<String>) -> Result<Config, ConfigError> {
        let solana_rpc_url = get("SOLANA_RPC_URL").ok_or(ConfigError::Missing("SOLANA_RPC_URL"))?;
        let database_url = get("DATABASE_URL").ok_or(ConfigError::Missing("DATABASE_URL"))?;
        let jwt_secret = get("JWT_SECRET").ok_or(ConfigError::Missing("JWT_SECRET"))?;
        Ok(Config {
            solana_rpc_url,
            database_url,
            jwt_secret,
        })
    }

    pub fn from_env() -> Result<Config, ConfigError> {
        dotenvy::dotenv()
            .or_else(|_| dotenvy::from_filename("crates/api/.env"))
            .ok();
        Self::from_source(|key| std::env::var(key).ok())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn missing_env_var() {
        let result = Config::from_source(|_| Option::<String>::None).unwrap_err();
        assert_eq!(result, ConfigError::Missing("SOLANA_RPC_URL"));
    }

    #[test]
    fn happy_path_config() {
        let result = Config::from_source(|_| Some("dummy".to_string()));
        assert_eq!(
            result,
            Ok(Config {
                solana_rpc_url: "dummy".to_string(),
                database_url: "dummy".to_string(),
                jwt_secret: "dummy".to_string()
            })
        );
    }
}

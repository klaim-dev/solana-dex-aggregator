use std::env;

#[derive(Debug)]
pub struct Config {
    solana_rpc_client: String,
    database_url: String,
    jwt_secret: String,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingEnvVar{reason: String},
}

impl Config {
   pub  fn from_env() -> Result<Config, ConfigError> {
        let solana_rpc_client = env::var("SOLANA_RPC_CLIENT").map_err(|_| ConfigError::MissingEnvVar { reason: "invalid solana rpc client".to_string() })?;
        let database_url = env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnvVar { reason: "invalid database url".to_string() })?;
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| ConfigError::MissingEnvVar { reason: "invalid jwt secret".to_string() })?;
        let config = Config { solana_rpc_client, database_url, jwt_secret };
        Ok(config)
    }

    pub fn get_solana_rpc_client(&self) -> String {
        self.solana_rpc_client.clone()
    }
}

pub struct AppState {
    pub config: Config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_env_var() {
        let err = Config::from_env().unwrap_err();
        assert!(matches!(err, ConfigError::MissingEnvVar { .. }));
    } 
}

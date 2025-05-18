use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use crate::errors::{ServiceError, ServiceResult};

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

impl PostgresConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MongoConfig {
    pub uri: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub postgres: PostgresConfig,
    pub mongodb: MongoConfig,
}

impl AppConfig {
    pub fn from_env() -> ServiceResult<Self> {
        dotenv().ok();
        
        let server_config = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .map_err(|e| ServiceError::ConfigError(format!("Invalid server port: {}", e)))?,
        };
        
        let postgres_config = PostgresConfig {
            host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("POSTGRES_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .map_err(|e| ServiceError::ConfigError(format!("Invalid postgres port: {}", e)))?,
            username: env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
            database: env::var("POSTGRES_DB").unwrap_or_else(|_| "business_service".to_string()),
            max_connections: env::var("POSTGRES_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .map_err(|e| ServiceError::ConfigError(format!("Invalid max connections: {}", e)))?,
        };
        
        let mongodb_config = MongoConfig {
            uri: env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            database: env::var("MONGODB_DATABASE").unwrap_or_else(|_| "business_service".to_string()),
        };
        
        Ok(AppConfig {
            server: server_config,
            postgres: postgres_config,
            mongodb: mongodb_config,
        })
    }
}
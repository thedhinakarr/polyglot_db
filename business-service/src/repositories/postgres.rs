use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::config::PostgresConfig;
use crate::errors::{ServiceError, ServiceResult};

pub struct PostgresClient {
    pub pool: PgPool,
}

impl PostgresClient {
    pub async fn new(config: &PostgresConfig) -> ServiceResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.connection_string())
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        Ok(Self { pool })
    }
    
    pub async fn health_check(&self) -> ServiceResult<bool> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| true)
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))
    }
}
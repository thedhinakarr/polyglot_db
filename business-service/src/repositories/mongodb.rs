use mongodb::{Client, Database, options::ClientOptions};
use crate::config::MongoConfig;
use crate::errors::{ServiceError, ServiceResult};

pub struct MongoClient {
    pub client: Client,
    pub database: Database,
}

impl MongoClient {
    pub async fn new(config: &MongoConfig) -> ServiceResult<Self> {
        let client_options = ClientOptions::parse(&config.uri)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        let client = Client::with_options(client_options)
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        let database = client.database(&config.database);
        
        Ok(Self { client, database })
    }
    
    pub async fn health_check(&self) -> ServiceResult<bool> {
        self.database
            .run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await
            .map(|_| true)
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))
    }
}
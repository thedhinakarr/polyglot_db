use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Entity not found: {0}")]
    NotFoundError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

pub type ServiceResult<T> = Result<T, ServiceError>;
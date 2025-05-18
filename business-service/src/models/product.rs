use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub sku: String,
    pub category: String,
    pub in_stock: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(name: String, description: String, price: f64, sku: String, category: String) -> Self {
        let now = Utc::now();
        Self {
            id: Some(Uuid::new_v4()),
            name,
            description,
            price,
            sku,
            category,
            in_stock: true,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub sku: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub sku: Option<String>,
    pub category: Option<String>,
    pub in_stock: Option<bool>,
}
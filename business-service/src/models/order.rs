use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "shipped")]
    Shipped,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub customer_id: Uuid,
    pub items: Vec<OrderItem>,
    pub total: f64,
    pub status: OrderStatus,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn new(customer_id: Uuid, items: Vec<OrderItem>) -> Self {
        let now = Utc::now();
        let total = items.iter().fold(0.0, |acc, item| acc + (item.price * item.quantity as f64));
        
        Self {
            id: Some(Uuid::new_v4()),
            customer_id,
            items,
            total,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderDto {
    pub customer_id: Uuid,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderStatusDto {
    pub status: OrderStatus,
}
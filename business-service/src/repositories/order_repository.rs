use async_trait::async_trait;
use sqlx::Row;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::order::{Order, OrderItem, OrderStatus};
use crate::repositories::{Repository, PostgresClient};

pub struct OrderRepository {
    pg_client: PostgresClient,
}

impl OrderRepository {
    pub fn new(pg_client: PostgresClient) -> Self {
        Self { pg_client }
    }

    async fn setup_tables(&self) -> ServiceResult<()> {
        // Create orders table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS orders (
                id UUID PRIMARY KEY,
                customer_id UUID NOT NULL,
                total DECIMAL(10, 2) NOT NULL,
                status VARCHAR(20) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL
            )
            "#
        )
        .execute(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        // Create order_items table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS order_items (
                id UUID PRIMARY KEY,
                order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
                product_id UUID NOT NULL,
                quantity INTEGER NOT NULL,
                price DECIMAL(10, 2) NOT NULL
            )
            "#
        )
        .execute(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        Ok(())
    }
    
    // Helper method to convert status string to enum
    fn status_from_str(status: &str) -> OrderStatus {
        match status {
            "pending" => OrderStatus::Pending,
            "processing" => OrderStatus::Processing,
            "shipped" => OrderStatus::Shipped,
            "delivered" => OrderStatus::Delivered,
            "cancelled" => OrderStatus::Cancelled,
            _ => OrderStatus::Pending,
        }
    }
    
    // Helper method to convert status enum to string
    fn status_to_str(status: &OrderStatus) -> &'static str {
        match status {
            OrderStatus::Pending => "pending",
            OrderStatus::Processing => "processing",
            OrderStatus::Shipped => "shipped",
            OrderStatus::Delivered => "delivered",
            OrderStatus::Cancelled => "cancelled",
        }
    }
}

#[async_trait]
impl Repository<Order, Uuid> for OrderRepository {
    async fn find_by_id(&self, id: Uuid) -> ServiceResult<Option<Order>> {
        // First fetch the order
        let order = sqlx::query(
            r#"
            SELECT id, customer_id, total, status, created_at, updated_at
            FROM orders
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        if let Some(order_row) = order {
            // Extract order fields
            let order_id: Uuid = order_row.try_get("id")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let customer_id: Uuid = order_row.try_get("customer_id")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let total: f64 = order_row.try_get::<f64, _>("total")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let status: String = order_row.try_get("status")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let created_at: DateTime<Utc> = order_row.try_get("created_at")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let updated_at: DateTime<Utc> = order_row.try_get("updated_at")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

            // Then fetch its items
            let items = sqlx::query(
                r#"
                SELECT product_id, quantity, price
                FROM order_items
                WHERE order_id = $1
                "#
            )
            .bind(id)
            .fetch_all(&self.pg_client.pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

            let order_items: Vec<OrderItem> = items
                .into_iter()
                .map(|item| -> Result<OrderItem, ServiceError> {
                    let product_id: Uuid = item.try_get("product_id")
                        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
                    let quantity: i32 = item.try_get("quantity")
                        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
                    let price: f64 = item.try_get::<f64, _>("price")
                        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
                    
                    Ok(OrderItem {
                        product_id,
                        quantity,
                        price,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            let order = Order {
                id: Some(order_id),
                customer_id,
                items: order_items,
                total,
                status: Self::status_from_str(&status),
                created_at,
                updated_at,
            };

            Ok(Some(order))
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> ServiceResult<Vec<Order>> {
        // Fetch all orders
        let orders = sqlx::query(
            r#"
            SELECT id, customer_id, total, status, created_at, updated_at
            FROM orders
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let mut result = Vec::with_capacity(orders.len());

        // For each order, fetch its items
        for order_row in orders {
            let order_id: Uuid = order_row.try_get("id")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let customer_id: Uuid = order_row.try_get("customer_id")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let total: f64 = order_row.try_get::<f64, _>("total")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let status: String = order_row.try_get("status")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let created_at: DateTime<Utc> = order_row.try_get("created_at")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            let updated_at: DateTime<Utc> = order_row.try_get("updated_at")
                .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

            let items = sqlx::query(
                r#"
                SELECT product_id, quantity, price
                FROM order_items
                WHERE order_id = $1
                "#
            )
            .bind(order_id)
            .fetch_all(&self.pg_client.pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

            let order_items: Vec<OrderItem> = items
                .into_iter()
                .map(|item| -> Result<OrderItem, ServiceError> {
                    let product_id: Uuid = item.try_get("product_id")
                        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
                    let quantity: i32 = item.try_get("quantity")
                        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
                    let price: f64 = item.try_get::<f64, _>("price")
                        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
                    
                    Ok(OrderItem {
                        product_id,
                        quantity,
                        price,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            let order = Order {
                id: Some(order_id),
                customer_id,
                items: order_items,
                total,
                status: Self::status_from_str(&status),
                created_at,
                updated_at,
            };

            result.push(order);
        }

        Ok(result)
    }

    async fn create(&self, item: Order) -> ServiceResult<Order> {
        let transaction = self.pg_client.pool.begin().await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let id = item.id.unwrap_or_else(Uuid::new_v4);
        
        // Insert the order
        sqlx::query(
            r#"
            INSERT INTO orders (id, customer_id, total, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(id)
        .bind(item.customer_id)
        .bind(item.total)
        .bind(Self::status_to_str(&item.status))
        .bind(item.created_at)
        .bind(item.updated_at)
        .execute(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        // Insert all order items
        for item_data in &item.items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO order_items (id, order_id, product_id, quantity, price)
                VALUES ($1, $2, $3, $4, $5)
                "#
            )
            .bind(item_id)
            .bind(id)
            .bind(item_data.product_id)
            .bind(item_data.quantity)
            .bind(item_data.price)
            .execute(&self.pg_client.pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
        }

        // Commit the transaction
        transaction.commit().await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let mut created_item = item;
        created_item.id = Some(id);

        Ok(created_item)
    }

    async fn update(&self, id: Uuid, item: Order) -> ServiceResult<Order> {
        // Check if order exists
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Err(ServiceError::NotFoundError(format!("Order with ID {} not found", id)));
        }

        let transaction = self.pg_client.pool.begin().await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        // Update the order
        sqlx::query(
            r#"
            UPDATE orders
            SET total = $1, status = $2, updated_at = $3
            WHERE id = $4
            "#
        )
        .bind(item.total)
        .bind(Self::status_to_str(&item.status))
        .bind(item.updated_at)
        .bind(id)
        .execute(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        // Delete existing items and insert new ones
        sqlx::query(
            r#"
            DELETE FROM order_items
            WHERE order_id = $1
            "#
        )
        .bind(id)
        .execute(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        // Insert all order items
        for item_data in &item.items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO order_items (id, order_id, product_id, quantity, price)
                VALUES ($1, $2, $3, $4, $5)
                "#
            )
            .bind(item_id)
            .bind(id)
            .bind(item_data.product_id)
            .bind(item_data.quantity)
            .bind(item_data.price)
            .execute(&self.pg_client.pool)
            .await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
        }

        // Commit the transaction
        transaction.commit().await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        let mut updated_item = item;
        updated_item.id = Some(id);

        Ok(updated_item)
    }

    async fn delete(&self, id: Uuid) -> ServiceResult<()> {
        // Check if order exists
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Err(ServiceError::NotFoundError(format!("Order with ID {} not found", id)));
        }

        // The items will be deleted automatically due to ON DELETE CASCADE
        sqlx::query(
            r#"
            DELETE FROM orders
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&self.pg_client.pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
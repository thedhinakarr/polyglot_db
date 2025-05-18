use uuid::Uuid;
use crate::errors::ServiceResult;
use crate::models::order::{Order, CreateOrderDto, UpdateOrderStatusDto};
use crate::repositories::{Repository, OrderRepository};

pub struct OrderService {
    repository: OrderRepository,
}

impl OrderService {
    pub fn new(repository: OrderRepository) -> Self {
        Self { repository }
    }
    
    pub async fn get_order(&self, id: Uuid) -> ServiceResult<Option<Order>> {
        self.repository.find_by_id(id).await
    }
    
    pub async fn get_all_orders(&self) -> ServiceResult<Vec<Order>> {
        self.repository.find_all().await
    }
    
    pub async fn create_order(&self, dto: CreateOrderDto) -> ServiceResult<Order> {
        let order = Order::new(
            dto.customer_id,
            dto.items,
        );
        
        self.repository.create(order).await
    }
    
    pub async fn update_order_status(&self, id: Uuid, dto: UpdateOrderStatusDto) -> ServiceResult<Order> {
        // First, get the existing order
        let existing_order = self.repository.find_by_id(id).await?
            .ok_or_else(|| crate::errors::ServiceError::NotFoundError(format!("Order with id {} not found", id)))?;
        
        // Create updated order with new status
        let mut updated_order = existing_order.clone();
        updated_order.status = dto.status;
        updated_order.updated_at = chrono::Utc::now();
        
        self.repository.update(id, updated_order).await
    }
    
    pub async fn delete_order(&self, id: Uuid) -> ServiceResult<()> {
        self.repository.delete(id).await
    }
}
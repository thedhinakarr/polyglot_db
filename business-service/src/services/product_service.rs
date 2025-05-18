use uuid::Uuid;
use crate::errors::ServiceResult;
use crate::models::product::{Product, CreateProductDto, UpdateProductDto};
use crate::repositories::{Repository, ProductRepository};

pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Self {
        Self { repository }
    }
    
    pub async fn get_product(&self, id: Uuid) -> ServiceResult<Option<Product>> {
        self.repository.find_by_id(id).await
    }
    
    pub async fn get_all_products(&self) -> ServiceResult<Vec<Product>> {
        self.repository.find_all().await
    }
    
    pub async fn create_product(&self, dto: CreateProductDto) -> ServiceResult<Product> {
        let product = Product::new(
            dto.name,
            dto.description,
            dto.price,
            dto.sku,
            dto.category,
        );
        
        self.repository.create(product).await
    }
    
    pub async fn update_product(&self, id: Uuid, dto: UpdateProductDto) -> ServiceResult<Product> {
        // First, get the existing product
        let existing_product = self.repository.find_by_id(id).await?
            .ok_or_else(|| crate::errors::ServiceError::NotFoundError(format!("Product with id {} not found", id)))?;
        
        // Create updated product with values from DTO or existing values
        let updated_product = Product {
            id: existing_product.id,
            name: dto.name.unwrap_or(existing_product.name),
            description: dto.description.unwrap_or(existing_product.description),
            price: dto.price.unwrap_or(existing_product.price),
            sku: dto.sku.unwrap_or(existing_product.sku),
            category: dto.category.unwrap_or(existing_product.category),
            in_stock: dto.in_stock.unwrap_or(existing_product.in_stock),
            created_at: existing_product.created_at,
            updated_at: chrono::Utc::now(),
        };
        
        self.repository.update(id, updated_product).await
    }
    
    pub async fn delete_product(&self, id: Uuid) -> ServiceResult<()> {
        self.repository.delete(id).await
    }
}
use async_trait::async_trait;
use futures_util::StreamExt; // Change to StreamExt instead of TryStreamExt
use mongodb::bson::{doc, from_document, to_document};
use mongodb::Collection;
use uuid::Uuid;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::product::Product;
use crate::repositories::{Repository, MongoClient};

pub struct ProductRepository {
    mongo_client: MongoClient,
    collection_name: String,
}

impl ProductRepository {
    pub fn new(mongo_client: MongoClient) -> Self {
        Self {
            mongo_client,
            collection_name: "products".to_string(),
        }
    }
    
    // Helper method to get the typed collection
    fn collection(&self) -> Collection<mongodb::bson::Document> {
        self.mongo_client.database.collection(&self.collection_name)
    }
}

#[async_trait]
impl Repository<Product, Uuid> for ProductRepository {
    async fn find_by_id(&self, id: Uuid) -> ServiceResult<Option<Product>> {
        let collection = self.collection();
        
        let filter = doc! { "_id": id.to_string() };
        let result = collection.find_one(filter, None).await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        match result {
            Some(document) => {
                let mut product: Product = from_document(document)
                    .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
                product.id = Some(id);
                Ok(Some(product))
            },
            None => Ok(None),
        }
    }
    
    async fn find_all(&self) -> ServiceResult<Vec<Product>> {
        let collection = self.collection();
        
        let cursor = collection.find(None, None).await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        // Use StreamExt::collect to convert the stream into a vector
        let documents: Vec<Result<mongodb::bson::Document, _>> = cursor.collect().await;
        
        // Process the results
        let mut products = Vec::new();
        for doc_result in documents {
            match doc_result {
                Ok(doc) => {
                    match from_document::<Product>(doc) {
                        Ok(product) => products.push(product),
                        Err(e) => return Err(ServiceError::DatabaseError(e.to_string())),
                    }
                }
                Err(e) => return Err(ServiceError::DatabaseError(e.to_string())),
            }
        }
        
        Ok(products)
    }
    
    async fn create(&self, item: Product) -> ServiceResult<Product> {
        let collection = self.collection();
        
        let id = item.id.unwrap_or_else(Uuid::new_v4);
        let mut document = to_document(&item)
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        document.insert("_id", id.to_string());
        
        collection.insert_one(document, None).await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        let mut created_item = item;
        created_item.id = Some(id);
        
        Ok(created_item)
    }
    
    async fn update(&self, id: Uuid, item: Product) -> ServiceResult<Product> {
        let collection = self.collection();
        
        let filter = doc! { "_id": id.to_string() };
        let mut document = to_document(&item)
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        // Remove _id from the document (we don't want to update it)
        document.remove("_id");
        
        let update = doc! { "$set": document };
        
        let result = collection.update_one(filter, update, None).await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        if result.matched_count == 0 {
            return Err(ServiceError::NotFoundError(format!("Product with ID {} not found", id)));
        }
        
        let mut updated_item = item;
        updated_item.id = Some(id);
        
        Ok(updated_item)
    }
    
    async fn delete(&self, id: Uuid) -> ServiceResult<()> {
        let collection = self.collection();
        
        let filter = doc! { "_id": id.to_string() };
        let result = collection.delete_one(filter, None).await
            .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;
            
        if result.deleted_count == 0 {
            return Err(ServiceError::NotFoundError(format!("Product with ID {} not found", id)));
        }
        
        Ok(())
    }
}
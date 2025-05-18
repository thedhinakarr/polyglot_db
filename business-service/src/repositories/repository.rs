use async_trait::async_trait;
use crate::errors::ServiceResult;

#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> ServiceResult<Option<T>>;
    async fn find_all(&self) -> ServiceResult<Vec<T>>;
    async fn create(&self, item: T) -> ServiceResult<T>;
    async fn update(&self, id: ID, item: T) -> ServiceResult<T>;
    async fn delete(&self, id: ID) -> ServiceResult<()>;
}
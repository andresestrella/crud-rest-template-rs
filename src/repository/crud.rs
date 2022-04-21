use async_trait::async_trait;
use super::RepositoryError;

#[async_trait]
pub trait Crud <T>{
    async fn get(&self, id:u64) -> Result<T, RepositoryError>; 
    async fn get_all(&self) -> Result<Vec<T>, RepositoryError>; 
    async fn create(&self, data:&T) -> Result<T, RepositoryError>; 
    async fn update(&self, data:&T) -> Result<T, RepositoryError>; 
    async fn delete(&self, id:u64) -> Result<T, RepositoryError>; 
}
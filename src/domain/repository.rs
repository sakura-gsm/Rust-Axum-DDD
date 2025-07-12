use super::todo::{Todo, TodoId};
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Todo>, RepositoryError>;
    async fn find_by_id(&self, id: &TodoId) -> Result<Option<Todo>, RepositoryError>;
    async fn save(&self, todo: Todo) -> Result<Todo, RepositoryError>;
    async fn delete(&self, id: &TodoId) -> Result<(), RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Todo not found")]
    NotFound,
}

use std::sync::Arc;
use thiserror::Error;

use crate::domain::{RepositoryError, Todo, TodoId, TodoRepository, TodoStatus};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Todo not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Clone)]
pub struct TodoService {
    repository: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>, ServiceError> {
        self.repository.find_all().await.map_err(ServiceError::from)
    }

    pub async fn get_todo_by_id(&self, id: &TodoId) -> Result<Todo, ServiceError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or(ServiceError::NotFound)
    }

    pub async fn create_todo(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, ServiceError> {
        if title.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Title cannot be empty".to_string(),
            ));
        }

        let todo = Todo::new(title, description);
        self.repository.save(todo).await.map_err(ServiceError::from)
    }

    pub async fn update_todo_status(
        &self,
        id: &TodoId,
        status: TodoStatus,
    ) -> Result<Todo, ServiceError> {
        let mut todo = self.get_todo_by_id(id).await?;
        todo.update_status(status);
        self.repository.save(todo).await.map_err(ServiceError::from)
    }

    pub async fn update_todo_content(
        &self,
        id: &TodoId,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, ServiceError> {
        if title.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Title cannot be empty".to_string(),
            ));
        }

        let mut todo = self.get_todo_by_id(id).await?;
        todo.update_content(title, description);
        self.repository.save(todo).await.map_err(ServiceError::from)
    }

    pub async fn delete_todo(&self, id: &TodoId) -> Result<(), ServiceError> {
        let _ = self.get_todo_by_id(id).await?; // Check if exists
        self.repository.delete(id).await.map_err(ServiceError::from)
    }
}

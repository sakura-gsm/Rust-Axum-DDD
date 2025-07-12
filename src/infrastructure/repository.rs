use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::domain::{RepositoryError, Todo, TodoId, TodoRepository};

pub struct InMemoryTodoRepository {
    todos: Arc<Mutex<HashMap<Uuid, Todo>>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl TodoRepository for InMemoryTodoRepository {
    async fn find_all(&self) -> Result<Vec<Todo>, RepositoryError> {
        let todos = self.todos.lock().unwrap();
        Ok(todos.values().cloned().collect())
    }

    async fn find_by_id(&self, id: &TodoId) -> Result<Option<Todo>, RepositoryError> {
        let todos = self.todos.lock().unwrap();
        Ok(todos.get(&id.0).cloned())
    }

    async fn save(&self, todo: Todo) -> Result<Todo, RepositoryError> {
        let mut todos = self.todos.lock().unwrap();
        todos.insert(todo.id.0, todo.clone());
        Ok(todo)
    }

    async fn delete(&self, id: &TodoId) -> Result<(), RepositoryError> {
        let mut todos = self.todos.lock().unwrap();
        todos.remove(&id.0);
        Ok(())
    }
}

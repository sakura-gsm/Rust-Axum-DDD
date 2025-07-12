// === src/main.rs ===
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::service::TodoService;
use infrastructure::repository::InMemoryTodoRepository;
use presentation::handler::*;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService,
}

#[tokio::main]
async fn main() {
    // Initialize repository and service
    let repository = Arc::new(InMemoryTodoRepository::new());
    let todo_service = TodoService::new(repository);
    let app_state = Arc::new(AppState { todo_service });

    // Build router
    let app = Router::new()
        .route("/todos", axum::routing::get(get_todos).post(create_todo))
        .route(
            "/todos/{id}",
            axum::routing::get(get_todo)
                .put(update_todo)
                .delete(delete_todo),
        )
        .route("/todos/{id}/status", axum::routing::put(update_todo_status))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}

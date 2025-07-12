use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::service::ServiceError,
    domain::{Todo, TodoId, TodoStatus},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoStatusRequest {
    pub status: TodoStatus,
}

pub async fn get_todos(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Todo>>, StatusCode> {
    match state.todo_service.get_all_todos().await {
        Ok(todos) => Ok(Json(todos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, StatusCode> {
    let todo_id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let todo_id = TodoId(todo_id);

    match state.todo_service.get_todo_by_id(&todo_id).await {
        Ok(todo) => Ok(Json(todo)),
        Err(ServiceError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    match state
        .todo_service
        .create_todo(req.title, req.description)
        .await
    {
        Ok(todo) => Ok(Json(todo)),
        Err(ServiceError::InvalidInput(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let todo_id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let todo_id = TodoId(todo_id);

    match state
        .todo_service
        .update_todo_content(&todo_id, req.title, req.description)
        .await
    {
        Ok(todo) => Ok(Json(todo)),
        Err(ServiceError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(ServiceError::InvalidInput(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_todo_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTodoStatusRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let todo_id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let todo_id = TodoId(todo_id);

    match state
        .todo_service
        .update_todo_status(&todo_id, req.status)
        .await
    {
        Ok(todo) => Ok(Json(todo)),
        Err(ServiceError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let todo_id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let todo_id = TodoId(todo_id);

    match state.todo_service.delete_todo(&todo_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(ServiceError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

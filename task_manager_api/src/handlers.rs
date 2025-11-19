use crate::errors::ApiError;
use crate::models::{CreateTaskRequest, HealthResponse, Task, UpdateTaskRequest};
use crate::repository::TaskRepository;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".to_string(),
    })
}

pub async fn create_task(
    State(repo): State<Arc<TaskRepository>>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    if payload.title.trim().is_empty() {
        return Err(ApiError::ValidationError(
            "Title can not be empty".to_string(),
        ));
    }
    let task = repo.create(payload);
    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn list_tasks(State(repo): State<Arc<TaskRepository>>) -> Json<Vec<Task>> {
    Json(repo.list())
}

pub async fn get_task(
    State(repo): State<Arc<TaskRepository>>,
    Path(id): Path<u64>,
) -> Result<Json<Task>, ApiError> {
    repo.get(id).map(Json).ok_or(ApiError::NotFound)
}

pub async fn update_task(
    State(repo): State<Arc<TaskRepository>>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, ApiError> {
    repo.update(id, payload).map(Json).ok_or(ApiError::NotFound)
}

pub async fn delete_task(
    State(repo): State<Arc<TaskRepository>>,
    Path(id): Path<u64>,
) -> Result<StatusCode, ApiError> {
    if repo.delete(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

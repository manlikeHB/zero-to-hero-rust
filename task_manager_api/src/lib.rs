use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

mod errors;
mod handlers;
mod models;
mod repository;
mod sort;
mod utils;

use handlers::{create_task, delete_task, get_task, health_check, list_tasks, update_task};
pub use repository::TaskRepository;

pub fn create_app(repo: Arc<TaskRepository>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/tasks", post(create_task).get(list_tasks))
        .route(
            "/tasks/:id",
            get(get_task).patch(update_task).delete(delete_task),
        )
        .with_state(repo)
}

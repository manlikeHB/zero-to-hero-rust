use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

use task_manager_api::{TaskRepository, create_app};

/// Helper function to create a fresh app for each test
fn create_test_app() -> axum::Router {
    let repo = Arc::new(TaskRepository::new());
    create_app(repo)
}

/// Helper to parse JSON response body
async fn parse_json_body(body: Body) -> serde_json::Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

// ===== Test: Health Check =====

#[tokio::test]
async fn test_health_check() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["status"], "OK");
}

// ===== Test: Create Task =====

#[tokio::test]
async fn test_create_task() {
    let app = create_test_app();

    let payload = json!({
        "title": "Test Task",
        "description": "Test Description"
    });

    let request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Should return 201 Created
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = parse_json_body(response.into_body()).await;

    // Verify task fields
    assert_eq!(body["id"], 1);
    assert_eq!(body["title"], "Test Task");
    assert_eq!(body["description"], "Test Description");
    assert_eq!(body["completed"], false);
    assert!(body["created_at"].is_string());
}

#[tokio::test]
async fn test_create_task_without_description() {
    let app = create_test_app();

    let payload = json!({
        "title": "Task without description"
    });

    let request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["description"], serde_json::Value::Null);
}

// ===== Test: List Tasks =====

#[tokio::test]
async fn test_list_tasks_empty() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/tasks")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json_body(response.into_body()).await;
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_list_tasks_with_data() {
    let app = create_test_app();

    // Create first task
    let payload1 = json!({"title": "Task 1"});
    let request1 = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload1.to_string()))
        .unwrap();
    app.clone().oneshot(request1).await.unwrap();

    // Create second task
    let payload2 = json!({"title": "Task 2"});
    let request2 = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload2.to_string()))
        .unwrap();
    app.clone().oneshot(request2).await.unwrap();

    // List all tasks
    let request = Request::builder()
        .uri("/tasks")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json_body(response.into_body()).await;
    let tasks = body.as_array().unwrap();

    assert_eq!(tasks.len(), 2);

    let titles: Vec<&str> = tasks.iter().map(|t| t["title"].as_str().unwrap()).collect();

    assert!(titles.contains(&"Task 1"));
    assert!(titles.contains(&"Task 2"));
}

// ===== Test: Get Single Task =====

#[tokio::test]
async fn test_get_task_success() {
    let app = create_test_app();

    // Create a task first
    let payload = json!({"title": "Get Me"});
    let create_request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    app.clone().oneshot(create_request).await.unwrap();

    // Get the task
    let request = Request::builder()
        .uri("/tasks/1")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["id"], 1);
    assert_eq!(body["title"], "Get Me");
}

#[tokio::test]
async fn test_get_task_not_found() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/tasks/999")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"], "Resource not found");
}

// ===== Test: Update Task =====

#[tokio::test]
async fn test_update_task_title() {
    let app = create_test_app();

    // Create a task
    let payload = json!({"title": "Original", "description": "Original desc"});
    let create_request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    app.clone().oneshot(create_request).await.unwrap();

    // Update only the title
    let update_payload = json!({"title": "Updated Title"});
    let update_request = Request::builder()
        .uri("/tasks/1")
        .method(Method::PATCH)
        .header("content-type", "application/json")
        .body(Body::from(update_payload.to_string()))
        .unwrap();

    let response = app.oneshot(update_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["title"], "Updated Title");
    assert_eq!(body["description"], "Original desc"); // Unchanged
}

#[tokio::test]
async fn test_update_task_clear_description() {
    let app = create_test_app();

    // Create a task with description
    let payload = json!({"title": "Task", "description": "Has description"});
    let create_request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    app.clone().oneshot(create_request).await.unwrap();

    // Clear the description
    let update_payload = json!({"description": null});
    let update_request = Request::builder()
        .uri("/tasks/1")
        .method(Method::PATCH)
        .header("content-type", "application/json")
        .body(Body::from(update_payload.to_string()))
        .unwrap();

    let response = app.oneshot(update_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["description"], serde_json::Value::Null);
}

#[tokio::test]
async fn test_update_task_mark_completed() {
    let app = create_test_app();

    // Create a task
    let payload = json!({"title": "Task"});
    let create_request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    app.clone().oneshot(create_request).await.unwrap();

    // Mark as completed
    let update_payload = json!({"completed": true});
    let update_request = Request::builder()
        .uri("/tasks/1")
        .method(Method::PATCH)
        .header("content-type", "application/json")
        .body(Body::from(update_payload.to_string()))
        .unwrap();

    let response = app.oneshot(update_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["completed"], true);
}

#[tokio::test]
async fn test_update_nonexistent_task() {
    let app = create_test_app();

    let payload = json!({"title": "Won't work"});
    let request = Request::builder()
        .uri("/tasks/999")
        .method(Method::PATCH)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = parse_json_body(response.into_body()).await;
    assert_eq!(body["error"], "Resource not found");
}

// ===== Test: Delete Task =====

#[tokio::test]
async fn test_delete_task_success() {
    let app = create_test_app();

    // Create a task
    let payload = json!({"title": "Delete Me"});
    let create_request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    app.clone().oneshot(create_request).await.unwrap();

    // Delete it
    let delete_request = Request::builder()
        .uri("/tasks/1")
        .method(Method::DELETE)
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(delete_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let get_request = Request::builder()
        .uri("/tasks/1")
        .body(Body::empty())
        .unwrap();

    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent_task() {
    let app = create_test_app();

    let request = Request::builder()
        .uri("/tasks/999")
        .method(Method::DELETE)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ===== Test: Complex Workflow =====

#[tokio::test]
async fn test_full_crud_workflow() {
    let app = create_test_app();

    // 1. List should be empty
    let list_request = Request::builder()
        .uri("/tasks")
        .body(Body::empty())
        .unwrap();
    let list_response = app.clone().oneshot(list_request).await.unwrap();
    let tasks = parse_json_body(list_response.into_body()).await;
    assert_eq!(tasks.as_array().unwrap().len(), 0);

    // 2. Create a task
    let create_payload = json!({"title": "Workflow Task", "description": "Test workflow"});
    let create_request = Request::builder()
        .uri("/tasks")
        .method(Method::POST)
        .header("content-type", "application/json")
        .body(Body::from(create_payload.to_string()))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);

    // 3. Get the task
    let get_request = Request::builder()
        .uri("/tasks/1")
        .body(Body::empty())
        .unwrap();
    let get_response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);

    // 4. Update the task
    let update_payload = json!({"completed": true});
    let update_request = Request::builder()
        .uri("/tasks/1")
        .method(Method::PATCH)
        .header("content-type", "application/json")
        .body(Body::from(update_payload.to_string()))
        .unwrap();
    let update_response = app.clone().oneshot(update_request).await.unwrap();
    let updated_task = parse_json_body(update_response.into_body()).await;
    assert_eq!(updated_task["completed"], true);

    // 5. Delete the task
    let delete_request = Request::builder()
        .uri("/tasks/1")
        .method(Method::DELETE)
        .body(Body::empty())
        .unwrap();
    let delete_response = app.clone().oneshot(delete_request).await.unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    // 6. Verify list is empty again
    let final_list_request = Request::builder()
        .uri("/tasks")
        .body(Body::empty())
        .unwrap();
    let final_list_response = app.oneshot(final_list_request).await.unwrap();
    let final_tasks = parse_json_body(final_list_response.into_body()).await;
    assert_eq!(final_tasks.as_array().unwrap().len(), 0);
}

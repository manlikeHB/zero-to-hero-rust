use std::sync::Arc;
use task_manager_api::TaskRepository;
use task_manager_api::create_app;

#[tokio::main]
async fn main() {
    let repo = Arc::new(TaskRepository::new());

    let app = create_app(repo);

    println!("Starting server on port 3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("PORT 3000 already in use");
    axum::serve(listener, app).await.expect("Server error");
}

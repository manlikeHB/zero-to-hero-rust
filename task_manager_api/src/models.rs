use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTaskRequest {
    #[serde(default, deserialize_with = "deserialize_some")]
    pub title: Option<String>,

    #[serde(default, deserialize_with = "deserialize_some")]
    pub description: Option<Option<String>>,

    #[serde(default, deserialize_with = "deserialize_some")]
    pub completed: Option<bool>,
}

// Helper function for deserializing Option<Option<T>>
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Deserialize)]
pub struct TaskQuery {
    pub completed: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_serialization() {
        let task = Task {
            id: 1,
            title: "Learn Rust".to_string(),
            description: Some("Build REST API".to_string()),
            completed: false,
            created_at: "2025-10-30T14:23:45Z".to_string(),
        };

        let json = serde_json::to_string(&task).unwrap();

        // Verify all fields are present in JSON
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"title\":\"Learn Rust\""));
        assert!(json.contains("\"description\":\"Build REST API\""));
        assert!(json.contains("\"completed\":false"));
        assert!(json.contains("\"created_at\":\"2025-10-30T14:23:45Z\""));
    }

    #[test]
    fn test_task_with_null_description() {
        let task = Task {
            id: 2,
            title: "Task without description".to_string(),
            description: None,
            completed: true,
            created_at: "2025-10-30T14:23:45Z".to_string(),
        };

        let json = serde_json::to_string(&task).unwrap();

        // Should serialize None as null
        assert!(json.contains("\"description\":null"));
        assert!(json.contains("\"completed\":true"));
    }

    #[test]
    fn test_task_deserialization() {
        let json = r#"{
            "id": 3,
            "title": "Test Task",
            "description": "Test Description",
            "completed": false,
            "created_at": "2025-10-30T14:23:45Z"
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();

        assert_eq!(task.id, 3);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, Some("Test Description".to_string()));
        assert_eq!(task.completed, false);
        assert_eq!(task.created_at, "2025-10-30T14:23:45Z");
    }

    #[test]
    fn test_task_deserialization_null_description() {
        let json = r#"{
            "id": 4,
            "title": "No description",
            "description": null,
            "completed": true,
            "created_at": "2025-10-30T14:23:45Z"
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();

        assert_eq!(task.description, None);
    }

    #[test]
    fn test_create_request_with_description() {
        let json = r#"{
            "title": "New Task",
            "description": "Task details"
        }"#;

        let req: CreateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.title, "New Task");
        assert_eq!(req.description, Some("Task details".to_string()));
    }

    #[test]
    fn test_create_request_without_description() {
        let json = r#"{"title": "Minimal Task"}"#;

        let req: CreateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.title, "Minimal Task");
        assert_eq!(req.description, None);
    }

    #[test]
    fn test_create_request_with_null_description() {
        let json = r#"{
            "title": "Task with null",
            "description": null
        }"#;

        let req: CreateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(req.description, None);
    }

    #[test]
    fn test_create_request_missing_title_fails() {
        let json = r#"{"description": "No title"}"#;

        let result: Result<CreateTaskRequest, _> = serde_json::from_str(json);

        assert!(result.is_err(), "Should fail when title is missing");
    }

    #[test]
    fn test_create_request_serialization() {
        let req = CreateTaskRequest {
            title: "Serialize test".to_string(),
            description: Some("With description".to_string()),
        };

        let json = serde_json::to_string(&req).unwrap();

        assert!(json.contains("\"title\":\"Serialize test\""));
        assert!(json.contains("\"description\":\"With description\""));
    }
}

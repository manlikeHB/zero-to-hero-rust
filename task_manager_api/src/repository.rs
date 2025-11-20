use crate::models::{
    CreateTaskRequest, PaginationMeta, PaginationResponse, Task, UpdateTaskRequest,
};
use crate::sort::{SortField, SortOrder};
use crate::utils;
use chrono;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct TaskRepository {
    tasks: Arc<Mutex<HashMap<u64, Task>>>,
    next_id: Arc<Mutex<u64>>,
}

impl TaskRepository {
    pub fn new() -> Self {
        TaskRepository {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    pub fn create(&self, req: CreateTaskRequest) -> Task {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            *next_id += 1;
            *next_id
        };

        let task = Task {
            id,
            title: req.title,
            description: req.description,
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.insert(id, task.clone());
        }

        task
    }

    pub fn list(
        &self,
        completed: Option<bool>,
        search: Option<String>,
        sort_by: Option<SortField>,
        order: SortOrder,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> PaginationResponse<Task> {
        let tasks = {
            self.tasks
                .lock()
                .unwrap()
                .values()
                .cloned()
                .collect::<Vec<Task>>()
        };

        let tasks = utils::filter_and_sort(tasks, completed, search, sort_by, order);
        let total = tasks.len() as u32;

        let (tasks, page_num, limit_num, total_pages) = if let (Some(p), Some(l)) = (page, limit) {
            let paginated = utils::paginate(tasks, p, l);
            let total_pages = if total == 0 {
                0 // No items = no pages
            } else {
                (total as f32 / l as f32).ceil() as u32
            };
            (paginated, p, l, total_pages)
        } else {
            // No pagination: all items on "page 1"
            let total_pages = if total == 0 { 0 } else { 1 };
            (tasks, 1, total, total_pages)
        };

        PaginationResponse {
            data: tasks,
            pagination: PaginationMeta {
                page: page_num,
                limit: limit_num,
                total,
                total_pages,
            },
        }
    }

    pub fn get(&self, id: u64) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.get(&id).cloned()
    }

    pub fn update(&self, id: u64, req: UpdateTaskRequest) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();

        tasks.get_mut(&id).map(|task| {
            if let Some(title) = req.title {
                task.title = title;
            }

            if let Some(description) = req.description {
                task.description = description;
            }

            if let Some(completed) = req.completed {
                task.completed = completed;
            }

            task.clone()
        })
    }

    pub fn delete(&self, id: u64) -> bool {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(&id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_get_task() {
        let repo = TaskRepository::new();
        let req = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("This is a test task".to_string()),
        };

        let task = repo.create(req);
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description.as_deref(), Some("This is a test task"));
        assert!(!task.completed);

        let fetched_task = repo.get(1).unwrap();
        assert_eq!(fetched_task.id, 1);
    }

    #[test]
    fn test_list_tasks() {
        let repo = TaskRepository::new();
        let req1 = CreateTaskRequest {
            title: "Task 1".to_string(),
            description: None,
        };
        let req2 = CreateTaskRequest {
            title: "Task 2".to_string(),
            description: Some("Second task".to_string()),
        };

        repo.create(req1);
        repo.create(req2);

        let tasks = repo
            .list(None, None, None, SortOrder::default(), None, None)
            .data;
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_concurrent_creates() {
        let repo = Arc::new(TaskRepository::new());
        let mut handles = vec![];

        // spawn 10 threads, each creating 10 tasks
        for _i in 0..10 {
            let repo_clone = Arc::clone(&repo);
            let handle = std::thread::spawn(move || {
                for i in 0..10 {
                    let req = CreateTaskRequest {
                        title: format!("Task {}-{}", i, i),
                        description: None,
                    };

                    repo_clone.create(req);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let tasks = repo
            .list(None, None, None, SortOrder::default(), None, None)
            .data;
        assert_eq!(tasks.len(), 100);

        let mut ids: Vec<u64> = tasks.iter().map(|t| t.id).collect();
        ids.sort();
        ids.dedup();

        assert_eq!(ids.len(), 100); // ensure all IDs are unique
    }

    #[test]
    fn test_update_task() {
        let repo = TaskRepository::new();
        let req1 = CreateTaskRequest {
            title: "Task 1".to_string(),
            description: None,
        };

        let task1 = repo.create(req1);

        let tasks = repo
            .list(None, None, None, SortOrder::default(), None, None)
            .data;
        assert_eq!(tasks.len(), 1);

        let update_title_req = UpdateTaskRequest {
            title: Some("Updated Task 1".to_string()),
            description: Some(None),
            completed: None,
        };

        let updated_task = repo.update(task1.id, update_title_req);
        assert!(updated_task.is_some());
        let updated_task = updated_task.unwrap();
        assert_eq!(updated_task.title, "Updated Task 1");
        assert_eq!(updated_task.description, None);
        assert_eq!(updated_task.completed, false);

        let update_desc_req = UpdateTaskRequest {
            title: None,
            description: Some(Some("Now has description".to_string())),
            completed: Some(true),
        };

        let updated_task = repo.update(task1.id, update_desc_req);
        assert!(updated_task.is_some());
        let updated_task = updated_task.unwrap();
        // println!("Updated Task: {:?}", updated_task);
        assert_eq!(
            updated_task.description.as_deref(),
            Some("Now has description")
        );
        assert_eq!(updated_task.completed, true);

        let update_desc_req = UpdateTaskRequest {
            title: None,
            description: Some(Some("Now has description".to_string())),
            completed: Some(true),
        };

        let updated_task = repo.update(task1.id, update_desc_req);
        assert!(updated_task.is_some());
        let updated_task = updated_task.unwrap();
        println!("Updated Task: {:?}", updated_task);
        assert_eq!(
            updated_task.description.as_deref(),
            Some("Now has description")
        );
        assert_eq!(updated_task.completed, true);

        let update_desc_req = UpdateTaskRequest {
            title: None,
            description: None,
            completed: None,
        };

        let updated_task = repo.update(6, update_desc_req);
        assert!(updated_task.is_none());
    }

    #[test]
    fn test_list_filtered_task() {
        let repo = TaskRepository::new();
        let req1 = CreateTaskRequest {
            title: "Test Task 1".to_string(),
            description: Some("This is a test task".to_string()),
        };
        let req2 = CreateTaskRequest {
            title: "Test Task 2".to_string(),
            description: Some("This is a test task".to_string()),
        };

        let task1 = repo.create(req1);
        let task2 = repo.create(req2);

        let updated_task1 = repo.update(
            task1.id,
            UpdateTaskRequest {
                title: None,
                description: None,
                completed: Some(true),
            },
        );
        assert!(updated_task1.is_some());

        let completed_tasks = repo
            .list(Some(true), None, None, SortOrder::default(), None, None)
            .data;
        assert_eq!(completed_tasks.len(), 1);
        assert_eq!(completed_tasks[0].id, task1.id);

        let incomplete_tasks = repo
            .list(
                Some(false),
                Some("test".to_string()),
                None,
                SortOrder::default(),
                None,
                None,
            )
            .data;
        assert_eq!(incomplete_tasks.len(), 1);
        assert_eq!(incomplete_tasks[0].id, task2.id);

        let all_tasks = repo
            .list(None, None, None, SortOrder::default(), None, None)
            .data;

        assert_eq!(all_tasks.len(), 2);

        let non_existing_task = repo
            .list(
                Some(false),
                Some("Nonexisting".to_string()),
                None,
                SortOrder::default(),
                None,
                None,
            )
            .data;
        assert_eq!(non_existing_task.len(), 0);
    }
}

use crate::models::Task;
use crate::sort::{SortField, SortOrder};

pub fn filter_and_sort(
    tasks: Vec<Task>,
    completed: Option<bool>,
    search: Option<String>,
    sort_by: Option<SortField>,
    order: SortOrder,
) -> Vec<Task> {
    let mut filtered: Vec<Task> = tasks
        .into_iter()
        .filter(|task| {
            let task_completed = completed.map_or(true, |filter| task.completed == filter);

            let task_search = search.as_ref().map_or(true, |term| {
                task.title.to_lowercase().contains(&term.to_lowercase())
            });

            task_completed && task_search
        })
        .collect();

    if let Some(field) = sort_by {
        filtered.sort_by(|a, b| {
            let cmp = match field {
                SortField::Title => a.title.to_lowercase().cmp(&b.title.to_lowercase()),
                SortField::CreatedAt => a.created_at.cmp(&b.created_at),
                SortField::Completed => a.completed.cmp(&b.completed),
            };

            match order {
                SortOrder::Asc => cmp,
                SortOrder::Desc => cmp.reverse(),
            }
        });
    }

    filtered
}

pub fn paginate(tasks: Vec<Task>, page: u32, limit: u32) -> Vec<Task> {
    let offset = (page.saturating_sub(1)) * limit;
    let paginated = tasks
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    paginated
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::{SortField, SortOrder};

    fn create_test_task(id: u64, title: &str, completed: bool) -> Task {
        Task {
            id,
            title: title.to_string(),
            description: None,
            completed,
            created_at: "2025-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_filter_by_completed() {
        let tasks = vec![
            create_test_task(1, "Task 1", true),
            create_test_task(2, "Task 2", false),
            create_test_task(3, "Task 3", true),
        ];

        let filtered = filter_and_sort(
            tasks,
            Some(true), // Only completed
            None,
            None,
            SortOrder::Asc,
        );

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].id, 1);
        assert_eq!(filtered[1].id, 3);
    }

    #[test]
    fn test_filter_by_search() {
        let tasks = vec![
            create_test_task(1, "Learn Rust", false),
            create_test_task(2, "Learn Python", false),
            create_test_task(3, "Build Rust API", false),
        ];

        let filtered = filter_and_sort(tasks, None, Some("rust".to_string()), None, SortOrder::Asc);

        assert_eq!(filtered.len(), 2);
        assert!(filtered[0].title.to_lowercase().contains("rust"));
    }

    #[test]
    fn test_sort_by_title() {
        let tasks = vec![
            create_test_task(1, "Zebra", false),
            create_test_task(2, "Apple", false),
            create_test_task(3, "Mango", false),
        ];

        let sorted = filter_and_sort(tasks, None, None, Some(SortField::Title), SortOrder::Asc);

        assert_eq!(sorted[0].title, "Apple");
        assert_eq!(sorted[1].title, "Mango");
        assert_eq!(sorted[2].title, "Zebra");
    }

    #[test]
    fn test_paginate() {
        let tasks = vec![
            create_test_task(1, "Task 1", false),
            create_test_task(2, "Task 2", false),
            create_test_task(3, "Task 3", false),
            create_test_task(4, "Task 4", false),
            create_test_task(5, "Task 5", false),
        ];

        let page1 = paginate(tasks.clone(), 1, 2);
        assert_eq!(page1.len(), 2);
        assert_eq!(page1[0].id, 1);

        let page2 = paginate(tasks, 2, 2);
        assert_eq!(page2.len(), 2);
        assert_eq!(page2[0].id, 3);
    }
}

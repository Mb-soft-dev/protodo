use protodo::db::TaskStore;
use protodo::task::TaskStatus;

// Integration tests for the `db` module

#[test]
fn test_get_by_id() {
    let task_store = TaskStore::new().expect("Failed to initialize TaskStore");
    task_store.clear_all_tasks().unwrap();

    let description = "Task to find".to_string();
    let task = task_store.add(description.clone()).unwrap();

    let fetched_task = task_store.get_by_id(task.id).expect("Failed to fetch task");
    assert_eq!(fetched_task.id, task.id);
    assert_eq!(fetched_task.description, description);

    let non_existent_task = task_store.get_by_id(9999);
    assert!(non_existent_task.is_err()); // Expect an error for non-existent ID
}

#[test]
fn test_add_task() {
    let task_store = TaskStore::new().expect("Failed to initialize TaskStore");
    let description = "Test task description".to_string();

    let result = task_store.add(description.clone());
    assert!(result.is_ok());

    let task = result.unwrap();
    assert_eq!(task.description, description);
    assert_eq!(task.status, TaskStatus::Pending);
}

#[test]
fn test_delete_task() {
    let task_store = TaskStore::new().expect("Failed to initialize TaskStore");
    let task = task_store.add("Task to delete".to_string()).unwrap();

    let result = task_store.delete_task(task.id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), task.id);
}

#[test]
fn test_update_status() {
    let task_store = TaskStore::new().expect("Failed to initialize TaskStore");
    let task = task_store.add("Task to update".to_string()).unwrap();

    let result = task_store.update_status(task.id, TaskStatus::Done);
    assert!(result.is_ok());

    let updated_task = task_store
        .list_tasks_with_pagination(0, 100)
        .unwrap()
        .into_iter()
        .find(|t| t.id == task.id)
        .unwrap();
    assert_eq!(updated_task.status, TaskStatus::Done);
}

#[test]
fn test_pagination() {
    let task_store = TaskStore::new().expect("Failed to initialize TaskStore");
    task_store.clear_all_tasks().unwrap(); // Clear stale tasks

    // Add 50 tasks
    for i in 1..=50 {
        task_store
            .add(format!("Task {i:02}"))
            .expect("Failed to add task");
    }

    // Test page 1 with page_size 10 (should return tasks 1-10)
    let tasks = task_store
        .list_tasks_with_pagination(0, 10)
        .expect("Failed to list tasks with pagination");
    assert_eq!(tasks.len(), 10);
    assert_eq!(tasks[0].description, "Task 01");
    assert_eq!(tasks[9].description, "Task 10");

    // Test page 2 with page_size 10 (should return tasks 11-20)
    let tasks = task_store
        .list_tasks_with_pagination(10, 10)
        .expect("Failed to list tasks with pagination");
    assert_eq!(tasks.len(), 10);
    assert_eq!(tasks[0].description, "Task 11");
    assert_eq!(tasks[9].description, "Task 20");

    // Test page 3 with page_size 10 (should return tasks 21-25)
    let tasks = task_store
        .list_tasks_with_pagination(40, 10)
        .expect("Failed to list tasks with pagination");
    assert_eq!(tasks.len(), 10);
    assert_eq!(tasks[0].description, "Task 41");
    assert_eq!(tasks[4].description, "Task 45");

    // Test an empty page (e.g., page 4 with page_size 10, no tasks remain)
    let tasks = task_store
        .list_tasks_with_pagination(50, 10)
        .expect("Failed to list tasks with pagination");
    assert_eq!(tasks.len(), 0);
}

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

use protodo::db::TaskStore;
use protodo::task::TaskStatus;

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
fn test_list_tasks() {
    let task_store = TaskStore::new().expect("Failed to initialize TaskStore");
    task_store.clear_all_tasks().unwrap(); // Clear stale tasks
    task_store.clear_all_tasks().unwrap();
    task_store.add("Task A".to_string()).unwrap();
    task_store.add("Task B".to_string()).unwrap();

    let tasks = task_store.list_tasks().expect("Failed to list tasks");
    assert!(tasks.len() >= 2);
    assert_eq!(tasks[0].description, "Task A");
    assert_eq!(tasks[1].description, "Task B");
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
        .list_tasks()
        .unwrap()
        .into_iter()
        .find(|t| t.id == task.id)
        .unwrap();
    assert_eq!(updated_task.status, TaskStatus::Done);
}

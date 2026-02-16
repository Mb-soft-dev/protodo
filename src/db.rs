use crate::task::Task;

#[derive(Debug, Clone)]
pub struct TaskStore {
    pub tasks: Vec<Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add(&mut self, description: String) {
        let id = self.tasks.len() as u32 + 1;
        self.tasks.push(Task::new(id, description));
    }

    pub fn list(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn list_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }
}

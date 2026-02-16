#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    // InProgress,
    // Done,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending,
            completed: false,
        }
    }

    // pub fn find_by_id(tasks: &[Task], id: u32) -> Option<&Task> {
    //     tasks.iter().find(|task| task.id == id)
    // }
    //
    // pub fn mark_in_progress(&mut self) {
    //     self.status = TaskStatus::InProgress;
    // }
    //
    // pub fn mark_done(&mut self) {
    //     self.status = TaskStatus::Done;
    // }
}

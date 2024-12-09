use crate::task_state::TaskState;

pub struct Task {
    pub title: String,
    pub description: String,
    pub state: TaskState,
}

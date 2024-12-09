use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum TaskState {
    #[default]
    Todo = 0,
    InProgress = 1,
    Completed = 2,
    Outdated = 3,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

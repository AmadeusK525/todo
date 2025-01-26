mod operation;
mod renderer;
mod task;
mod task_manager;
mod task_state;
mod undo_tree;
mod writer;

use renderer::board_renderer::BoardRenderer;
use task::Task;
use task_manager::{create_task_manager, TaskData};
use task_state::TaskState;
use writer::writer::EmptyWriter;

fn main() {
    let mut terminal = ratatui::init();
    let tasks = Vec::from([
        Task {
            title: String::from("my task 0"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 2"),
            description: String::from("test"),
            state: TaskState::InProgress,
        },
        Task {
            title: String::from("my task 3"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 4"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 5"),
            description: String::from("test"),
            state: TaskState::Completed,
        },
        Task {
            title: String::from("my task 1"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 3"),
            description: String::from("test"),
            state: TaskState::InProgress,
        },
        Task {
            title: String::from("my task 4"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 5"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 6"),
            description: String::from("test"),
            state: TaskState::Completed,
        },
        Task {
            title: String::from("my task 1"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 3"),
            description: String::from("test"),
            state: TaskState::InProgress,
        },
        Task {
            title: String::from("my task 4"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 5"),
            description: String::from("test"),
            state: TaskState::Todo,
        },
        Task {
            title: String::from("my task 6"),
            description: String::from("test"),
            state: TaskState::Completed,
        },
    ]);
    let renderer = Box::new(BoardRenderer::default());

    let mut manager = create_task_manager(
        TaskData { tasks, focus: 0 },
        renderer,
        Box::new(EmptyWriter {}),
    );

    manager.run(&mut terminal);

    ratatui::restore();
}

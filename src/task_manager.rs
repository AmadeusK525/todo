use crate::{
    operation::Operation, renderer::renderer::Renderer, task::Task, undo_tree::UndoTree,
    writer::writer::Writer,
};
use crossterm::event::{self, KeyEvent};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

#[derive(Default)]
pub struct TaskData {
    pub tasks: Vec<Task>,
    pub focus: usize,
}

pub struct TaskManager {
    data: TaskData,

    renderer: Box<dyn Renderer>,
    writer: Box<dyn Writer>,

    undo_tree_move: UndoTree,
    undo_tree_op: UndoTree,

    quit: bool,
}

pub fn create_task_manager(
    data: TaskData,
    renderer: Box<dyn Renderer>,
    writer: Box<dyn Writer>,
) -> TaskManager {
    TaskManager {
        data,
        renderer,
        writer,
        undo_tree_move: UndoTree {},
        undo_tree_op: UndoTree {},
        quit: false,
    }
}

impl TaskManager {
    pub fn run(&mut self, term: &mut Terminal<CrosstermBackend<Stdout>>) {
        self.renderer.updated(&self.data, Operation::None);

        loop {
            let result = term.draw(|frame| {
                self.renderer.render(frame, &self.data);
            });

            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("something went wrong when rendering! '{}'", e);
                }
            }

            let e = event::read();
            match e {
                Ok(e) => {
                    self.handle_event(e);
                }
                Err(e) => {
                    eprintln!("something went wrong when reading an event! '{}'", e);
                }
            }

            if self.quit {
                break;
            }
        }
    }

    fn handle_event(&mut self, e: event::Event) {
        match e {
            event::Event::Key(key) => {
                self.handle_key_event(key);
            }
            _ => {}
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Char(char) => self.handle_char_event(char),
            _ => {}
        }
    }

    fn handle_char_event(&mut self, c: char) {
        let operation = match c {
            'h' => self.renderer.pressed_h(&self.data),
            'j' => self.renderer.pressed_j(&self.data),
            'k' => self.renderer.pressed_k(&self.data),
            'l' => self.renderer.pressed_l(&self.data),
            'H' => self.renderer.pressed_H(&self.data),
            'J' => self.renderer.pressed_J(&self.data),
            'K' => self.renderer.pressed_K(&self.data),
            'L' => self.renderer.pressed_L(&self.data),
            'q' => {
                self.quit = true;
                Operation::None
            }
            _ => Operation::None,
        };

        // TODO: Handle result?
        _ = self.perform_operation(operation);
    }

    fn perform_operation(&mut self, operation: Operation) -> Result<bool, String> {
        let result = match operation {
            Operation::None => Ok(false),
            Operation::Invalid => Err(String::from("tried performing invalid operation")),
            Operation::Move { from, to } => {
                if from == to {
                    return Ok(false);
                }

                if from >= self.data.tasks.len() {
                    return Err(String::from("move failed: 'from' idx out of bounds!"));
                }

                if to >= self.data.tasks.len() {
                    return Err(String::from("move failed: 'to' idx out of bounds!"));
                }

                let task = self.data.tasks.remove(from);
                let move_to = if from > to { to } else { to - 1 };

                self.data.tasks.insert(move_to, task);
                self.undo_tree_move.add_operation(Operation::Move {
                    from: (move_to),
                    to: (from),
                });

                Ok(true)
            }
            Operation::Swap(x, y) => {
                if x == y {
                    return Ok(false);
                }

                if x >= self.data.tasks.len() {
                    return Err(String::from("move failed: 'x' idx out of bounds!"));
                }

                if y >= self.data.tasks.len() {
                    return Err(String::from("move failed: 'y' idx out of bounds!"));
                }

                self.data.tasks.swap(x, y);
                self.undo_tree_move.add_operation(Operation::Swap(x, y));

                Ok(true)
            }
            Operation::Focus(idx) => {
                if idx == self.data.focus {
                    return Ok(false);
                }

                if idx >= self.data.tasks.len() {
                    return Err(String::from("focus failed: idx out of bounds!"));
                }

                self.data.focus = idx;

                Ok(true)
            }
        };

        match result {
            Ok(performed) => {
                if performed {
                    self.renderer.updated(&self.data, operation);
                }
            }
            Err(_) => {}
        }

        return result;
    }
}

use super::renderer::Renderer;
use crate::{operation::Operation, task_manager::TaskData, task_state::TaskState};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, ListState},
};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct BoardRenderer {
    // Use BTreeMap to sort based on TaskState
    task_idxs_by_state: BTreeMap<TaskState, Vec<usize>>,

    focus_col: TaskState,
    focus_list_idx: usize,
}

impl Renderer for BoardRenderer {
    fn render(&self, frame: &mut ratatui::Frame, data: &TaskData) {
        let col_constraints = self.task_idxs_by_state.iter().map(|_| Constraint::Fill(1));
        let col_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(frame.area());

        for (col_idx, (state, task_idxs)) in self.task_idxs_by_state.iter().enumerate() {
            let mut list_state = ListState::default();
            let mut list_items: Vec<Text> = Vec::with_capacity(task_idxs.len());

            for (list_idx, task_idx) in task_idxs.iter().enumerate() {
                list_items.push(Text::raw(data.tasks[*task_idx].title.clone()));
                if *task_idx == data.focus {
                    list_state.select(Some(list_idx));
                }
            }

            let list = List::new(list_items)
                .block(Block::bordered().title(state.to_string()))
                .style(Style::new().white())
                .highlight_style(Style::new().italic())
                .highlight_symbol(">> ")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);

            frame.render_stateful_widget(list, col_areas[col_idx], &mut list_state);
        }
    }

    fn updated(&mut self, data: &TaskData, _operation: crate::operation::Operation) {
        self.task_idxs_by_state.clear();

        for (idx, task) in data.tasks.iter().enumerate() {
            let list = self
                .task_idxs_by_state
                .entry(task.state)
                .or_insert_with(Vec::new);

            list.push(idx);

            if data.focus == idx {
                self.focus_col = task.state;
                self.focus_list_idx = list.len() - 1;
            }
        }
    }

    fn pressed_h(&self, data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_j(&self, _data: &TaskData) -> Operation {
        match self.task_idxs_by_state.get(&self.focus_col) {
            Some(task_idxs) => {
                if task_idxs.is_empty() {
                    Operation::None
                } else if self.focus_list_idx >= task_idxs.len() - 1 {
                    Operation::Focus(task_idxs[0])
                } else {
                    Operation::Focus(task_idxs[self.focus_list_idx + 1])
                }
            }
            None => Operation::None,
        }
    }

    fn pressed_k(&self, _data: &TaskData) -> Operation {
        match self.task_idxs_by_state.get(&self.focus_col) {
            Some(task_idxs) => {
                if task_idxs.is_empty() {
                    Operation::None
                } else if self.focus_list_idx == 0 {
                    Operation::Focus(task_idxs[task_idxs.len() - 1])
                } else {
                    Operation::Focus(task_idxs[self.focus_list_idx - 1])
                }
            }
            None => Operation::None,
        }
    }

    fn pressed_l(&self, data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_H(&self, data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_J(&self, data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_K(&self, data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_L(&self, data: &TaskData) -> Operation {
        return Operation::None;
    }
}

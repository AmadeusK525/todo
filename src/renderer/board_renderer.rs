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
    tasks_idxs_by_state: BTreeMap<TaskState, Vec<usize>>,

    focus_col: TaskState,
    focus_list_idx: usize,
}

impl Renderer for BoardRenderer {
    fn render(&self, frame: &mut ratatui::Frame, data: &TaskData) {
        let col_constraints = self.tasks_idxs_by_state.iter().map(|_| Constraint::Fill(1));
        let col_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(frame.area());

        for (col_idx, (state, task_idxs)) in self.tasks_idxs_by_state.iter().enumerate() {
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
        self.tasks_idxs_by_state.clear();

        for (idx, task) in data.tasks.iter().enumerate() {
            let list = self
                .tasks_idxs_by_state
                .entry(task.state)
                .or_insert_with(Vec::new);

            list.push(idx);

            if data.focus == idx {
                self.focus_col = task.state;
                self.focus_list_idx = list.len() - 1;
            }
        }
    }

    fn pressed_h(&self, _data: &TaskData) -> Operation {
        match self.tasks_idxs_by_state.keys().position(|k| k == &self.focus_col) {
            Some(focus_col_idx) => {
                let new_focus_col = if focus_col_idx == 0 {
                    self.tasks_idxs_by_state.keys().last()
                } else {
                    self.tasks_idxs_by_state.keys().nth(focus_col_idx - 1)
                }
                .unwrap();

                let new_focus_col_idxs = self.tasks_idxs_by_state.get(new_focus_col).unwrap();
                let new_focus_list_idx = self.focus_list_idx.clamp(0, new_focus_col_idxs.len() - 1);

                let new_focus_idx = new_focus_col_idxs[new_focus_list_idx];

                Operation::Focus(new_focus_idx)
            }
            None => Operation::None,
        }
    }

    fn pressed_j(&self, _data: &TaskData) -> Operation {
        match self.tasks_idxs_by_state.get(&self.focus_col) {
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
        match self.tasks_idxs_by_state.get(&self.focus_col) {
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

    fn pressed_l(&self, _data: &TaskData) -> Operation {
        match self.tasks_idxs_by_state.keys().position(|k| k == &self.focus_col) {
            Some(focus_col_idx) => {
                let new_focus_col = if focus_col_idx >= self.tasks_idxs_by_state.len() - 1 {
                    self.tasks_idxs_by_state.keys().nth(0)
                } else {
                    self.tasks_idxs_by_state.keys().nth(focus_col_idx + 1)
                }
                .unwrap();

                let new_focus_col_idxs = self.tasks_idxs_by_state.get(new_focus_col).unwrap();
                let new_focus_list_idx = self.focus_list_idx.clamp(0, new_focus_col_idxs.len() - 1);

                let new_focus_idx = new_focus_col_idxs[new_focus_list_idx];

                Operation::Focus(new_focus_idx)
            }
            None => Operation::None,
        }
    }

    fn pressed_H(&self, _data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_J(&self, _data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_K(&self, _data: &TaskData) -> Operation {
        return Operation::None;
    }

    fn pressed_L(&self, _data: &TaskData) -> Operation {
        return Operation::None;
    }
}

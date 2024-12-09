use crate::{operation::Operation, task_manager::TaskData};
use ratatui::Frame;

pub trait Renderer {
    fn updated(&mut self, data: &TaskData, operation: Operation);
    fn render(&self, frame: &mut Frame, data: &TaskData);

    fn pressed_h(&self, data: &TaskData) -> Operation;
    fn pressed_j(&self, data: &TaskData) -> Operation;
    fn pressed_k(&self, data: &TaskData) -> Operation;
    fn pressed_l(&self, data: &TaskData) -> Operation;

    fn pressed_H(&self, data: &TaskData) -> Operation;
    fn pressed_J(&self, data: &TaskData) -> Operation;
    fn pressed_K(&self, data: &TaskData) -> Operation;
    fn pressed_L(&self, data: &TaskData) -> Operation;
}

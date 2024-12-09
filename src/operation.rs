#[derive(Clone, Copy)]
pub enum Operation {
    Invalid,

    None,
    Move { from: usize, to: usize },
    Swap(usize, usize),
    Focus(usize),
}

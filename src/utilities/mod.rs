pub mod file;

/// Position used by tokens/errors
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub col: usize,
    pub line: usize
}

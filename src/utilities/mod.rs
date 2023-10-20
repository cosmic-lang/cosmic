/// Position used by tokens/errors
#[derive(Debug, PartialEq)]
pub struct Position {
  pub col: usize,
  pub line: usize
}

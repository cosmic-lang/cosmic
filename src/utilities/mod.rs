pub mod file;

/// Position used by tokens/errors
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub col: usize,
    pub line: usize
}

/// Returns true if char is a alphabetical char or '_'
pub fn is_alphabetical(ch: char) -> bool {
    return match ch {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false
    }
}

/// Returns true if char is a integer
pub fn is_integer(ch: char) -> bool {
    return match ch {
        '0'..='9' => true,
        _ => false
    }
}

/// Returns true if char is a number or '.'
pub fn is_numeric(ch: char) -> bool {
    return match ch {
        ch if is_integer(ch) => true,
        '.' => true,
        _ => false
    }
}

/// Returns true if char is a number or letter or '_'
pub fn is_alphanumeric(ch: char) -> bool {
    return match ch {
        ch if is_alphabetical(ch) | is_integer(ch) => true, 
        _ => false
    }
}
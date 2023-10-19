//!
//!

use super::token::Token;

///
pub struct Scanner<'a> {
  source: &'a str,
  read: usize,
  peek: usize,
  char: char
}

impl <'a> Scanner<'a> {
  /// Returns a new scanner
  pub fn new(source: &'a str) -> Scanner<'a> {
    let source = source.trim_start();
    Self {
      source,
      read: 0,
      peek: 1,
      char: source.chars().nth(0).unwrap()
    }
  }

  /// Advances the scanner count number of times
  fn advance(&mut self, count: usize) {
    let count = count.clamp(0, 2);

    for _ in 0..count {
      self.read += 1;
      self.peek += 1;

      self.char = match self.read >= self.source.len() {
        false => self.source.chars().nth(self.read).unwrap(),
        true => '\0'
      };
    }
  }

  /// Returns the next char
  fn peek(&self) -> char {
    return match self.peek >= self.source.len() {
      false => self.source.chars().nth(self.peek).unwrap(),
      true => '\0'
    }
  }

  /// Skips whitespace, preserves \n
  fn skip_whitespace(&mut self) {
    match self.char {
      ' ' | '\r' | '\t' => {
        self.advance(1);
        self.skip_whitespace()
      },
      _ => {}
    }
  }

  /// If # detected, skips until \n
  fn skip_comment(&mut self) {
    if self.char != '\n' {
      self.advance(1);
      self.skip_comment()
    }
  }

  /// Reads sequences of chars, used for tags and keywords
  fn read_tag(&mut self, start: usize, mut end: usize) -> &'a str {
    if is_alphanumeric(self.char) {
      self.advance(1);
      return self.read_tag(start, end + 1)
    }

    if self.char == '?' || self.char == '!' {
      self.advance(1);
      end += 1
    }

    return &self.source[start..end];
  }

  /// Reads integers and floats, if '.' is read, switches to read_float to read the decimals
  fn read_number(&mut self, start: usize, end: usize) -> &'a str {
    if self.char == '.' {
      self.advance(1);
      return self.read_float(start, end + 1)
    }
    else if is_numeric(self.char) {
      self.advance(1);
      return self.read_number(start, end + 1)
    }

    return &self.source[start..end];
  }
  
  /// Read the decimal portion of the float
  fn read_float(&mut self, start: usize, end: usize) -> &'a str {
    if is_integer(self.char) {
      self.advance(1);
      return self.read_float(start, end + 1)
    }

    return &self.source[start..end];
  }

  /// Scans the next token from the source
  pub fn next_token(&mut self) -> Token<'a> {
    self.skip_whitespace();

    return match self.char {
      // Tags and Keywords
      '#' => {
        self.skip_comment();
        self.next_token()
      }
      ch if is_alphabetical(ch) => {
        let chars = self.read_tag(self.read, self.read);
        return match Token::try_keyword(chars) {
          Some(token) => token,
          None => Token::Tag(chars)
        }
      },
      // Integers and Floats
      ch if is_integer(ch) => {
        let chars = self.read_number(self.read, self.read);

        return match chars.find('.') {
          Some(_) => {
            Token::Float(chars.parse::<f64>().unwrap())  
          },
          None => {
            Token::Integer(chars.parse::<i64>().unwrap())
          }
        }
      },
      // Compound operators
      '-' if self.peek() == '>' => {
        self.advance(2);
        Token::Arrow
      },
      '<' if self.peek() == '=' => {
        self.advance(2);
        Token::LesserEq
      },
      '>' if self.peek() == '=' => {
        self.advance(2);
        Token::GreaterEq
      },
      '!' if self.peek() == '=' => {
        self.advance(2);
        Token::NotEqual
      },
      '=' => {
        match self.peek() {
          '>' => { 
            self.advance(2);
            Token::FatArrow
          },
          '=' => { 
            self.advance(2);
            Token::Equal
          },
          _ => {
            self.advance(1);
            Token::of_char(self.char)
          }
        }
      },
      // All else
      ch => {
        self.advance(1);
        Token::of_char(ch)
      }
    }
  }
}

/// Returns true if char is a alphabetical char or '_'
fn is_alphabetical(ch: char) -> bool {
  return match ch {
    'a'..='z' | 'A'..='Z' | '_' => true,
    _ => false
  }
}

/// Returns true if char is a integer
fn is_integer(ch: char) -> bool {
  return match ch {
    '0'..='9' => true,
    _ => false
  }
}

/// Returns true if char is a number or '.'
fn is_numeric(ch: char) -> bool {
  return match ch {
    ch if is_integer(ch) => true,
    '.' => true,
    _ => false
  }
}

/// Returns true if char is a number or letter or '_'
fn is_alphanumeric(ch: char) -> bool {
  return match ch {
    ch if is_alphabetical(ch) | is_integer(ch) => true, 
    _ => false
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn scan_operators() {
    use crate::prelude::*;

    let source = "+-*/";
    let expected = [
      Token::Plus,
      Token::Minus,
      Token::Asterisk,
      Token::Slash
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token == Token::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    assert!(tokens == expected);
  }

  #[test]
  fn numbers() {
    use crate::prelude::*;

    let source = "
    # imma comment
    1234
    12.4 # Anothe Comment
    12.2.4
    ";

    let expected = [
      Token::Newline,
      Token::Integer(1234),
      Token::Newline,
      Token::Float(12.4),
      Token::Newline,
      Token::Float(12.2),
      Token::Dot,
      Token::Integer(4),
      Token::Newline
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token == Token::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    assert!(tokens == expected);
  }
  
  #[test]
  fn tags_n_keywords() {
    use crate::prelude::*;

    let source = "
    hello
    hey?
    yo!
    let
    ";

    let expected = [
      Token::Tag("hello"),
      Token::Newline,
      Token::Tag("hey?"),
      Token::Newline,
      Token::Tag("yo!"),
      Token::Newline,
      Token::Let,
      Token::Newline
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token == Token::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    assert!(tokens == expected);
  }
  
  #[test]
  fn compound_operators() {
    use crate::prelude::*;

    let source = "<==>->!!=";

    let expected = [
      Token::LesserEq,
      Token::FatArrow,
      Token::Arrow,
      Token::Bang,
      Token::NotEqual
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token == Token::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    assert!(tokens == expected);
  }
}

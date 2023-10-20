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
    let count = count.clamp(0, 3);

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
  
  /// Returns the next char at peek + count, skipping whitespace
  fn peek_plus(&self, count: usize) -> char {
    return match self.peek + count >= self.source.len() {
      false => self.source.chars().nth(self.peek + count).unwrap(),
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

  /// Reads strings
  fn read_string(&mut self, start: usize, end: usize) -> &'a str {
    if self.char != '"' && self.read < self.source.len() {
      self.advance(1);
      return self.read_string(start, end + 1);
    }

    self.advance(1);
    return &self.source[start..end];
  }
  
  /// Reads multiline strings
  fn read_multistring(&mut self, _start: usize, _end: usize) -> &'a str {
    return ""
  }

  /// Checks for compound operators
  fn compound_or_else(&mut self, rules: Vec<(char, Token<'a>)>, default: Token<'a>) -> Token<'a> {
    match self.char {
      ' ' | '\r' | '\t' => self.skip_whitespace(),
      _ => self.advance(1)
    }

    for rule in rules {
      if self.char == rule.0 {
        self.advance(1);
        return rule.1;
      }
    }

    return default;
  }
  
  /// Checks for three char compound operators
  fn compound_three(&mut self, rules: Vec<(char, char, Token<'a>)>) -> Option<Token<'a>> {
    for rule in rules {
      if self.peek() == rule.0 {
        if self.peek_plus(1) == rule.1 {
          self.advance(3);
          return Some(rule.2);
        }
      }
    }

    return None;
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
            Token::Float(chars)  
          },
          None => {
            Token::Integer(chars)
          }
        }
      },
      // Strings
      '"' => {
        self.advance(1);
        let chars = self.read_string(self.read, self.read);
        Token::String(chars)
      },
      // Multiline Strings
      '\\' if self.peek() == '\\' => {
        self.advance(2);
        let chars = self.read_multistring(self.read, self.read);
        Token::String(chars)
      },
      // Regex
      '`' => {
        Token::Regex("")
      }
      // Compound operators
      '=' => {
        self.compound_or_else(vec![
          ('>', Token::FatArrow),
          ('=', Token::Equal)
        ], Token::Assign)
      },
      '+' => { 
        self.compound_or_else(vec![
          ('+', Token::Increment)
        ], Token::Plus)
      },
      '*' => { 
        self.compound_or_else(vec![
          ('*', Token::Power)
        ], Token::Asterisk)
      },
      '-' => {
        self.compound_or_else(vec![
          ('>', Token::Arrow),
          ('-', Token::Decrement)
        ], Token::Minus)
      },
      '<' => {
        self.compound_or_else(vec![
          ('=', Token::LesserEq),
          ('<', Token::Lshift)
        ], Token::Lesser) 
      },
      '>' => {
        self.compound_or_else(vec![
          ('=', Token::GreaterEq),
          ('>', Token::Rshift)
        ], Token::Greater)  
      },
      '!' => {
        self.compound_or_else(vec![
          ('=', Token::NotEqual)
        ], Token::Bang) 
      },
      ':' => {
        self.compound_or_else(vec![
          ('=', Token::AssignExp)
        ], Token::Colon)
      },
      '|' => { 
        self.compound_or_else(vec![
          ('>', Token::Pipeline)
        ], Token::Pipe)
      },
      '.' => {
        // Check for three char operator
        match self.compound_three(vec![('.', '.', Token::RangeInc)]) {
          Some(token) => token,
          None => {
            // Check for two char operator
            self.compound_or_else(vec![
              ('.', Token::RangeExc)
            ], Token::Dot)    
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
  fn operators() {
    use crate::prelude::*;

    let source = "+-*/<>!@$%&^=|;:?,.";
    let expected = [
      Token::Plus,
      Token::Minus,
      Token::Asterisk,
      Token::Slash,
      Token::Lesser,
      Token::Greater,
      Token::Bang,
      Token::Address,
      Token::Cash,
      Token::Percent,
      Token::Ampersand,
      Token::Caret,
      Token::Assign,
      Token::Pipe,
      Token::Semicolon,
      Token::Colon,
      Token::Question,
      Token::Comma,
      Token::Dot
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
      Token::Integer("1234"),
      Token::Newline,
      Token::Float("12.4"),
      Token::Newline,
      Token::Float("12.2"),
      Token::Dot,
      Token::Integer("4"),
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
  fn strings() {
    use crate::prelude::*;

    let source = "
    \"12.2.1\"
    \"hello\"
    ";

    let expected = [
      Token::String("12.2.1"),
      Token::Newline,
      Token::String("hello"),
      Token::Newline,
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();
      dbg!(&token);
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

    let source = "<==>->!!=..:=...";

    let expected = [
      Token::LesserEq,
      Token::FatArrow,
      Token::Arrow,
      Token::Bang,
      Token::NotEqual,
      Token::RangeExc,
      Token::AssignExp,
      Token::RangeInc
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

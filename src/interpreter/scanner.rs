//!
//!

use super::token::{Token, TokenType};
use super::super::utilities::Position;

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
  
  /// Reads regex
  fn read_regex(&mut self, start: usize, end: usize) -> &'a str {
    if self.char != '`' && self.read < self.source.len() {
      self.advance(1);
      return self.read_regex(start, end + 1);
    }

    self.advance(1);
    return &self.source[start..end];
  }

  /// Checks for compound operators
  fn compound_or_else(&mut self, rules: Vec<(char, TokenType<'a>)>, default: TokenType<'a>) -> TokenType<'a> {
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
  fn compound_three(&mut self, rules: Vec<(char, char, TokenType<'a>)>) -> Option<TokenType<'a>> {
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

  /// Scans the next tokentype from the source
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
        return match TokenType::try_keyword(chars) {
          Some(tokentype) => {
            Token{
              tt: tokentype,
              pos: Position{col: 0, line: 0},
              file_name: ""
            }
          },
          None => {
            Token{
              tt: TokenType::Tag(chars),
              pos: Position{col: 0, line: 0},
              file_name: ""
            }
          }
        }
      },
      // Integers and Floats
      ch if is_integer(ch) => {
        let chars = self.read_number(self.read, self.read);

        return match chars.find('.') {
          Some(_) => {
            Token{
              tt: TokenType::Float(chars), 
              pos: Position{col: 0, line: 0},
              file_name: ""
            }
          },
          None => {
            Token{
              tt: TokenType::Integer(chars), 
              pos: Position{col: 0, line: 0},
              file_name: ""
            }
          }
        }
      },
      // Strings
      '"' => {
        self.advance(1);
        let chars = self.read_string(self.read, self.read);

        Token{
          tt: TokenType::String(chars), 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      // Multiline Strings
      '\\' if self.peek() == '\\' => {
        self.advance(2);
        let chars = self.read_multistring(self.read, self.read);
        
        Token{
          tt: TokenType::String(chars), 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      // Regex
      '`' => {
        self.advance(1);
        let chars = self.read_regex(self.read, self.read);

        Token{
          tt: TokenType::Regex(chars), 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      // Compound operators
      '=' => {
        let tt = self.compound_or_else(vec![
          ('>', TokenType::FatArrow),
          ('=', TokenType::Equal)
        ], TokenType::Assign);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '+' => { 
        let tt = self.compound_or_else(vec![
          ('+', TokenType::Increment)
        ], TokenType::Plus);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '*' => { 
        let tt = self.compound_or_else(vec![
          ('*', TokenType::Power)
        ], TokenType::Asterisk);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '-' => {
        let tt = self.compound_or_else(vec![
          ('>', TokenType::Arrow),
          ('-', TokenType::Decrement)
        ], TokenType::Minus);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '<' => {
        let tt = self.compound_or_else(vec![
          ('=', TokenType::LesserEq),
          ('<', TokenType::Lshift)
        ], TokenType::Lesser);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '>' => {
        let tt = self.compound_or_else(vec![
          ('=', TokenType::GreaterEq),
          ('>', TokenType::Rshift)
        ], TokenType::Greater); 
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '!' => {
        let tt = self.compound_or_else(vec![
          ('=', TokenType::NotEqual)
        ], TokenType::Bang);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      ':' => {
        let tt = self.compound_or_else(vec![
          ('=', TokenType::AssignExp)
        ], TokenType::Colon);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '|' => { 
        let tt = self.compound_or_else(vec![
          ('>', TokenType::Pipeline)
        ], TokenType::Pipe);
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      '.' => {
        // Check for three char operator
        let tt = match self.compound_three(vec![('.', '.', TokenType::RangeInc)]) {
          Some(tokentype) => tokentype,
          None => {
            // Check for two char operator
            self.compound_or_else(vec![
              ('.', TokenType::RangeExc)
            ], TokenType::Dot)    
          }
        };
        
        Token{
          tt, 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
      },
      // All else
      ch => {
        self.advance(1);
        Token{
          tt: TokenType::of_char(ch), 
          pos: Position{col: 0, line: 0},
          file_name: ""
        }
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
  use crate::prelude::*;

  fn test_tokens(tokens: &Vec<Token>, expected: &Vec<TokenType>) {
    assert!(tokens.len() == expected.len());

    for (t, e) in tokens.iter().zip(expected.iter()) {
      assert!(t.tt == *e)
    }
  }

  #[test]
  fn operators() {
    let source = "+-*/<>!@$%&^=|;:?,.";
    let expected = vec![
      TokenType::Plus,
      TokenType::Minus,
      TokenType::Asterisk,
      TokenType::Slash,
      TokenType::Lesser,
      TokenType::Greater,
      TokenType::Bang,
      TokenType::Address,
      TokenType::Cash,
      TokenType::Percent,
      TokenType::Ampersand,
      TokenType::Caret,
      TokenType::Assign,
      TokenType::Pipe,
      TokenType::Semicolon,
      TokenType::Colon,
      TokenType::Question,
      TokenType::Comma,
      TokenType::Dot
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token.tt == TokenType::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    test_tokens(&tokens, &expected)
  }

  #[test]
  fn numbers() {
    let source = "
    # imma comment
    1234
    12.4 # Anothe Comment
    12.2.4
    ";

    let expected = vec![
      TokenType::Newline,
      TokenType::Integer("1234"),
      TokenType::Newline,
      TokenType::Float("12.4"),
      TokenType::Newline,
      TokenType::Float("12.2"),
      TokenType::Dot,
      TokenType::Integer("4"),
      TokenType::Newline
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();
    
    loop {
      let token = scanner.next_token();

      if token.tt == TokenType::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    test_tokens(&tokens, &expected)
  }
  
  #[test]
  fn strings() {
    let source = "
    \"12.2.1\"
    \"hello\"
    ";

    let expected = vec![
      TokenType::String("12.2.1"),
      TokenType::Newline,
      TokenType::String("hello"),
      TokenType::Newline,
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token.tt == TokenType::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    test_tokens(&tokens, &expected)
  }
  
  #[test]
  fn tags_and_keywords() {
    let source = "
    hello
    hey?
    yo!
    let
    ";

    let expected = vec![
      TokenType::Tag("hello"),
      TokenType::Newline,
      TokenType::Tag("hey?"),
      TokenType::Newline,
      TokenType::Tag("yo!"),
      TokenType::Newline,
      TokenType::Let,
      TokenType::Newline
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token.tt == TokenType::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    test_tokens(&tokens, &expected)
  }
  
  #[test]
  fn compound_operators() {
    let source = "<==>->!!=..:=...";

    let expected = vec![
      TokenType::LesserEq,
      TokenType::FatArrow,
      TokenType::Arrow,
      TokenType::Bang,
      TokenType::NotEqual,
      TokenType::RangeExc,
      TokenType::AssignExp,
      TokenType::RangeInc
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();

      if token.tt == TokenType::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    test_tokens(&tokens, &expected)
  }
  
  #[test]
  fn assignment() {
    let source = "
    let x: int = 12
    const y: regex = `hello`
    ";

    let expected = vec![
      TokenType::Let,
      TokenType::Tag("x"),
      TokenType::Colon,
      TokenType::Tag("int"),
      TokenType::Assign,
      TokenType::Integer("12"),
      TokenType::Newline,
      TokenType::Const,
      TokenType::Tag("y"),
      TokenType::Colon,
      TokenType::Tag("regex"),
      TokenType::Assign,
      TokenType::Regex("hello"),
      TokenType::Newline
    ];
  
    let mut scanner = Scanner::new(&source);

    let mut tokens = Vec::new();

    loop {
      let token = scanner.next_token();
      dbg!(&token);

      if token.tt == TokenType::Eof {
        break;
      } else {
        tokens.push(token);
      }
    }

    test_tokens(&tokens, &expected)
  }
}

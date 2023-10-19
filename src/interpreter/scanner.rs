use super::token::Token;

pub struct Scanner<'a> {
  source: &'a str,
  read: usize,
  peek: usize,
  char: char
}

impl <'a> Scanner<'a> {
  pub fn new(source: &'a str) -> Scanner<'a> {
    Self {
      source,
      read: 0,
      peek: 1,
      char: source.chars().nth(0).unwrap()
    }
  }

  pub fn advance(&mut self) {
    self.read += 1;
    self.peek += 1;

    self.char = match self.read >= self.source.len() {
      false => self.source.chars().nth(self.read).unwrap(),
      true => '\0'
    };
  }

  pub fn read(&self) -> char {
    return self.char;
  }
  
  pub fn peek(&self) -> char {
    return match self.peek >= self.source.len() {
      false => self.source.chars().nth(self.peek).unwrap(),
      true => '\0'
    }
  }

  fn read_sequence(&mut self, func: fn (char) -> bool) -> &'a str {
    let start = self.read;
    let mut end = self.read + 1;
    while func(self.char) {
      end += 1; 
      self.advance();
    }

    return &self.source[start..end];
  }

  fn read_number(&mut self) -> &'a str {
    let start = self.read;
    let mut end = self.read + 1;
    while is_numeric(self.char) {
      end += 1; 
      self.advance();
    }

    return &self.source[start..end];
  }

  pub fn next_token(&mut self) -> Token<'a> {
    return match self.char {
      ch if is_alphabetical(ch) => {
        let chars = self.read_sequence(|ch| {return is_alphanumeric(ch)});
        return match Token::try_keyword(chars) {
          Some(token) => token,
          None => Token::Tag(chars)
        }
      },
      ch if is_integer(ch) => {
        let chars = self.read_number();

        return match chars.find('.') {
          Some(_) => {
            Token::Float(chars.parse::<f64>().unwrap())  
          },
          None => {
            Token::Integer(chars.parse::<i64>().unwrap())
          }
        }
      }
      ch => {
        self.advance();
        Token::of_char(ch)
      }
    }
  }
}

fn is_alphabetical(ch: char) -> bool {
  return match ch {
    'a'..='z' | 'A'..='Z' | '_' => true,
    _ => false
  }
}

fn is_integer(ch: char) -> bool {
  return match ch {
    '0'..='9' => true,
    _ => false
  }
}

fn is_numeric(ch: char) -> bool {
  return match ch {
    ch if is_integer(ch) => true,
    '.' => true,
    _ => false
  }
}

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
}

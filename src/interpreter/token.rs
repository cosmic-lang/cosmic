//!
//!

///
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  // Literals
  Tag(&'a str),
  Integer(i64),
  Float(f64),
  String(&'a str),
  Regex(&'a str),
  // Keywords
  Const,
  Let,
  Return,
  Fn,
  Record,
  Enum,
  Trait,
  Module,
  Defer,
  When,
  Inline,
  True,
  False,
  For,
  While,
  Break,
  Continue,
  Match,
  If,
  Else,
  As,
  And,
  Or,
  Dyn,
  Anytype,
  // Modes
  Mutable,
  Move,
  Local,
  Comptime,
  // Assignment
  Assign,
  AssignExp,
  // Punctuation
  Dot,
  Comma,
  Quote,
  DoubleQuote,
  Tick,
  Lparen,
  Rparen,
  Lbracket,
  Rbracket,
  Lsquirly,
  Rsquirly,
  Backslash,
  Colon,
  Semicolon,
  Arrow,
  FatArrow,
  // Operators
  Address,
  Cash,
  Pound,
  Bang,
  Question,
  RangeExc,
  RangeInc,
  // Arithmetic
  Plus,
  Minus,
  Asterisk,
  Slash,
  Percent,
  Increment,
  Decrement,
  Power,
  // Bitwise
  Ampersand,
  Pipe,
  Caret,
  Tilde,
  // Comparators
  Lesser,
  LesserEq,
  Greater,
  GreaterEq,
  Equal,
  NotEqual,
  // Others
  Newline,
  Illegal,
  Eof
}

impl <'a> Token<'a> {
  // If tag matches a keyword returns Some(that keyword), else None
  pub fn try_keyword(tag: &str) -> Option<Token> {
    match tag {
      "const"    => Some(Token::Const),
      "let"      => Some(Token::Let),
      "return"   => Some(Token::Return),
      "fn"       => Some(Token::Fn),
      "record"   => Some(Token::Record),
      "enum"     => Some(Token::Enum),
      "trait"    => Some(Token::Trait),
      "module"   => Some(Token::Module),
      "defer"    => Some(Token::Defer),
      "when"     => Some(Token::When),
      "inline"   => Some(Token::Inline),
      "true"     => Some(Token::True),
      "false"    => Some(Token::False),
      "for"      => Some(Token::For),
      "while"    => Some(Token::While),
      "break"    => Some(Token::Break),
      "continue" => Some(Token::Continue),
      "match"    => Some(Token::Match),
      "if"       => Some(Token::If),
      "else"     => Some(Token::Else),
      "as"       => Some(Token::As),
      "and"      => Some(Token::And),
      "or"       => Some(Token::Or),
      "dyn"      => Some(Token::Dyn),
      "anytype"  => Some(Token::Anytype),
      "mut"      => Some(Token::Mutable),
      "mov"      => Some(Token::Move),
      "loc"      => Some(Token::Local),
      "comptime" => Some(Token::Comptime),
      _ => None
    }
  }

  // Returns the Token eqivalent of char
  pub fn of_char(ch: char) -> Token<'a> {
    match ch {
      //
      '='  => Token::Assign,
      //
      '.'  => Token::Dot,
      ','  => Token::Comma,
      '\'' => Token::Quote,
      '\"' => Token::DoubleQuote,
      '`'  => Token::Tick,
      '('  => Token::Lparen,
      ')'  => Token::Rparen,
      '['  => Token::Lbracket,
      ']'  => Token::Rbracket,
      '{'  => Token::Lsquirly,
      '}'  => Token::Rsquirly,
      '\\' => Token::Backslash,
      ':'  => Token::Colon,
      ';'  => Token::Semicolon,
      //
      '@'  => Token::Address,
      '$'  => Token::Cash,
      '#'  => Token::Pound,
      '!'  => Token::Bang,
      '?'  => Token::Question,
      // 
      '+'  => Token::Plus,
      '-'  => Token::Minus,
      '*'  => Token::Asterisk,
      '/'  => Token::Slash,
      '%'  => Token::Percent,
      //
      '&'  => Token::Ampersand,
      '|'  => Token::Pipe,
      '^'  => Token::Caret,
      '~'  => Token::Tilde,
      //
      '<'  => Token::Lesser,
      '>'  => Token::Greater,
      //
      '\n' => Token::Newline,
      '\0' => Token::Eof,
      _ => Token::Illegal
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn keyword() {
    use crate::prelude::*;

    let source = "const let match hello".split(" ");
    let mut tokens = Vec::<Option<Token>>::new();
    let expected = [
      Some(Token::Const), 
      Some(Token::Let), 
      Some(Token::Match), 
      None
    ];
    
    for word in source {
      tokens.push(Token::try_keyword(word)); 
    }

    for i in 0..4 {
      assert!(tokens[i] == expected[i]);
    }
  }
}

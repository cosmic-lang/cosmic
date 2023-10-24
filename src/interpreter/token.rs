//!
//!

use std::rc::Rc;

use super::super::utilities::Position;

///
#[derive(Debug, PartialEq)]
pub struct Token<'a> {
  pub tt: TokenType<'a>,
  pub pos: Position,
  pub file_name: &'a str
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType<'a> {
  // Literals
  Tag(&'a str),
  Integer(&'a str),
  Float(&'a str),
  String(Rc<str>),
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
  Ctime,
  // Assignment
  Assign,
  AssignExp,
  // Punctuation
  Dot,
  Comma,
  Lparen,
  Rparen,
  Lbracket,
  Rbracket,
  Lsquirly,
  Rsquirly,
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
  Pipeline,
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
  Lshift,
  Rshift,
  // Comparators
  Lesser,
  LesserEq,
  Greater,
  GreaterEq,
  Equal,
  NotEqual,
  PatternMatch,
  PatternNotMatch,
  // Others
  Newline,
  Illegal,
  Eof
}

impl <'a> TokenType<'a> {
  // If tag matches a keyword returns Some(that keyword), else None
  pub fn try_keyword(tag: &str) -> Option<TokenType> {
    match tag {
      "const"    => Some(TokenType::Const),
      "let"      => Some(TokenType::Let),
      "return"   => Some(TokenType::Return),
      "fn"       => Some(TokenType::Fn),
      "record"   => Some(TokenType::Record),
      "enum"     => Some(TokenType::Enum),
      "trait"    => Some(TokenType::Trait),
      "module"   => Some(TokenType::Module),
      "defer"    => Some(TokenType::Defer),
      "when"     => Some(TokenType::When),
      "inline"   => Some(TokenType::Inline),
      "true"     => Some(TokenType::True),
      "false"    => Some(TokenType::False),
      "for"      => Some(TokenType::For),
      "while"    => Some(TokenType::While),
      "break"    => Some(TokenType::Break),
      "continue" => Some(TokenType::Continue),
      "match"    => Some(TokenType::Match),
      "if"       => Some(TokenType::If),
      "else"     => Some(TokenType::Else),
      "as"       => Some(TokenType::As),
      "and"      => Some(TokenType::And),
      "or"       => Some(TokenType::Or),
      "dyn"      => Some(TokenType::Dyn),
      "any"      => Some(TokenType::Anytype),
      "mut"      => Some(TokenType::Mutable),
      "mov"      => Some(TokenType::Move),
      "loc"      => Some(TokenType::Local),
      "ctime"    => Some(TokenType::Ctime),
      _ => None
    }
  }

  // Returns the TokenType eqivalent of char
  pub fn of_char(ch: char) -> TokenType<'a> {
    match ch {
      //
      '='  => TokenType::Assign,
      //
      '.'  => TokenType::Dot,
      ','  => TokenType::Comma,
      '('  => TokenType::Lparen,
      ')'  => TokenType::Rparen,
      '['  => TokenType::Lbracket,
      ']'  => TokenType::Rbracket,
      '{'  => TokenType::Lsquirly,
      '}'  => TokenType::Rsquirly,
      ':'  => TokenType::Colon,
      ';'  => TokenType::Semicolon,
      //
      '@'  => TokenType::Address,
      '$'  => TokenType::Cash,
      '#'  => TokenType::Pound,
      '!'  => TokenType::Bang,
      '?'  => TokenType::Question,
      // 
      '+'  => TokenType::Plus,
      '-'  => TokenType::Minus,
      '*'  => TokenType::Asterisk,
      '/'  => TokenType::Slash,
      '%'  => TokenType::Percent,
      //
      '&'  => TokenType::Ampersand,
      '|'  => TokenType::Pipe,
      '^'  => TokenType::Caret,
      '~'  => TokenType::Tilde,
      //
      '<'  => TokenType::Lesser,
      '>'  => TokenType::Greater,
      //
      '\n' => TokenType::Newline,
      '\0' => TokenType::Eof,
      _ => TokenType::Illegal
    }
  }

  // Returns the string representation of token
  pub fn to_string(&'a self) -> &'a str {
    match self {
      // Complex tokens
      TokenType::Tag(tag)        => tag,
      TokenType::Integer(int)    => int,
      TokenType::Float(float)    => float,
      TokenType::String(str)     => str,
      TokenType::Regex(reg)      => reg,
      // Keywords
      TokenType::Const           => "const",
      TokenType::Let             => "let",
      TokenType::Return          => "return",
      TokenType::Fn              => "fn",
      TokenType::Record          => "record",
      TokenType::Enum            => "enum",
      TokenType::Trait           => "trait",
      TokenType::Module          => "module",
      TokenType::Defer           => "defer",
      TokenType::When            => "when",
      TokenType::Inline          => "inline",
      TokenType::True            => "true",
      TokenType::False           => "false",
      TokenType::For             => "for",
      TokenType::While           => "while",
      TokenType::Break           => "break",
      TokenType::Continue        => "continue",
      TokenType::Match           => "match",
      TokenType::If              => "if",
      TokenType::Else            => "else",
      TokenType::As              => "as",
      TokenType::And             => "and",
      TokenType::Or              => "or",
      TokenType::Dyn             => "dyn",
      TokenType::Anytype         => "any",
      // Modes
      TokenType::Mutable         => "mut",
      TokenType::Move            => "mov",
      TokenType::Local           => "loc",
      TokenType::Ctime           => "ctime",
      // Assignment
      TokenType::Assign          => "=",
      TokenType::AssignExp       => ":=",
      // Puctuation
      TokenType::Dot             => ".",
      TokenType::Comma           => ",",
      TokenType::Lparen          => "(",
      TokenType::Rparen          => ")",
      TokenType::Lbracket        => "[",
      TokenType::Rbracket        => "]",
      TokenType::Lsquirly        => "{",
      TokenType::Rsquirly        => "}",
      TokenType::Colon           => ":",
      TokenType::Semicolon       => ";",
      TokenType::Arrow           => "->",
      TokenType::FatArrow        => "=>",
      // Operators
      TokenType::Address         => "@",
      TokenType::Cash            => "$",
      TokenType::Pound           => "#",
      TokenType::Bang            => "!",
      TokenType::Question        => "?",
      TokenType::RangeExc        => "..",
      TokenType::RangeInc        => "...",
      TokenType::Pipeline        => "|>",
      // Arithmetic
      TokenType::Plus            => "+",
      TokenType::Minus           => "-",
      TokenType::Asterisk        => "*",
      TokenType::Slash           => "/",
      TokenType::Percent         => "%",
      TokenType::Increment       => "++",
      TokenType::Decrement       => "--",
      TokenType::Power           => "**",
      // Bitwise
      TokenType::Ampersand       => "&",
      TokenType::Pipe            => "|",
      TokenType::Caret           => "^",
      TokenType::Tilde           => "~",
      TokenType::Lshift          => "<<",
      TokenType::Rshift          => ">>",
      // Comparators
      TokenType::Lesser          => "<",
      TokenType::LesserEq        => "<=",
      TokenType::Greater         => ">",
      TokenType::GreaterEq       => ">=",
      TokenType::Equal           => "==",
      TokenType::NotEqual        => "!=",
      TokenType::PatternMatch    => "=~",
      TokenType::PatternNotMatch => "!~",
      // Others
      TokenType::Newline         => "\n",
      TokenType::Illegal         => "illegal",
      TokenType::Eof             => "\0"
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn keyword() {
    use crate::prelude::*;

    let source = "const let match hello".split(" ");
    let mut tokens = Vec::<Option<TokenType>>::new();
    let expected = [
      Some(TokenType::Const), 
      Some(TokenType::Let), 
      Some(TokenType::Match), 
      None
    ];
    
    for word in source {
      tokens.push(TokenType::try_keyword(word)); 
    }

    for i in 0..4 {
      assert!(tokens[i] == expected[i]);
    }
  }
}

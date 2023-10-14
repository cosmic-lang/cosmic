//!
//!

const std = @import("std");

pub const Token = union(enum) {
  const Self = @This();
  // Literals
  tag: []const u8,
  integer: isize,
  float: std.meta.Tuple(&.{isize, isize}),
  string: []const u8,
  regex: []const u8,
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
  Anytype,
  And,
  Or,
  Dyn,
  // Modes
  Mutable,
  Move,
  Local,
  Comptime,
  // Assignment
  assign,
  assign_exp,
  // Punctuation
  dot,
  comma,
  quote,
  doublequote,
  tick,
  lparen,
  rparen,
  lbracket,
  rbracket,
  lbrace,
  rbrace,
  backslash,
  colon,
  semicolon,
  arrow,
  // Operators
  address,
  cash,
  pound,
  bang,
  question,
  range_exc,
  range_inc,
  range_all,
  // Mathmatical Operators
  plus,
  minus,
  asterisk,
  slash,
  percent,
  increment,
  decrement,
  power,
  // Bitwise Operators
  ampersand,
  pipe,
  caret,
  tilde,
  // Comparison Operators
  lesser,
  lessereq,
  greater,
  greatereq,
  equality,
  noteq,
  // Others
  newline,
  illegal,
  eof,

  //pub fn try_keyword(string: []const u8) ?Self {
  //    //@memcpy(tmp, string);
  //    inline for (@typeInfo(Token).Union.fields) |field| {
  //        if (std.mem.eql(u8, string, field.name)) {
  //            return @unionInit(Self, field.name, undefined); 
  //        }
  //    }
  //
  //    return null;
  //}

  pub fn of_char(char: u8) Token {
    return switch (char) {
      //
      '=' => Token.assign,
      //
      '.' => Token.dot,
      ',' => Token.comma,
      '\'' => Token.quote,
      '\"' => Token.doublequote,
      '`' => Token.tick,
      '(' => Token.lparen,
      ')' => Token.rparen,
      '[' => Token.lbracket,
      ']' => Token.rbracket,
      '{' => Token.lbrace,
      '}' => Token.rbrace,
      '\\' => Token.backslash,
      ':' => Token.colon,
      ';' => Token.semicolon,
      //
      '@' => Token.address,
      '$' => Token.cash,
      '#' => Token.pound,
      '!' => Token.bang,
      '?' => Token.question,
      //
      '+' => Token.plus,
      '-' => Token.minus,
      '*' => Token.asterisk,
      '/' => Token.slash,
      '%' => Token.percent,
      //
      '&' => Token.ampersand,
      '|' => Token.pipe,
      '^' => Token.caret,
      '~' => Token.tilde,
      //
      '<' => Token.lesser,
      '>' => Token.greater,
      //
      '\n' => Token.newline,
      '\x00' => Token.eof,
      else => Token.illegal
    };
  }
};

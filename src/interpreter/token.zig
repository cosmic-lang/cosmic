//!
//!

pub const Token = union(enum) {
  // Literals
  identifier: []u8,
  integer: isize,
  float: f32,
  string: []u8,
  tag: []u8,
  regex: []u8,
  // Keywords
  constant,
  let,
  ret,
  structure,
  enumeration,
  behaviour,
  true,
  false,
  do,
  end,
  for_loop,
  while_loop,
  brk,
  cont,
  match,
  if_cond,
  if_else,
  else_cond,
  // Assignment
  assign,
  assign_exp,
  // Modes
  uni,
  exc,
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
  pipe,
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
  noteq
};

//!
//!

const std = @import("std");

pub const Token = union(enum) {
    const Self = @This();
    // Literals
    tag: []const u8,
    integer: isize,
    float: isize, isize,
    string: []const u8,
    regex: []const u8,
    // Keywords
    Const,
    Let,
    Return,
    Def,
    Record,
    Variant,
    Behaviour,
    Module,
    True,
    False,
    DoLine,
    Do,
    End,
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
    // Modes
    Mut,
    Move,
    Local,
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

    pub fn try_keyword(string: []const u8) ?Token {
        if (std.mem.eql(u8, string, "const")) {
            return Token.Const; 
        } else if (std.mem.eql(u8, string, "let")) {
            return Token.Let; 
        } else {
            return null;
        }
    }

    pub fn of_char(char: u8) Token {
        return switch (char) {
            '=' => Token.assign,
            '\n' => Token.newline,
            '\x00' => Token.eof,
            else => Token.illegal
        };
    }
};

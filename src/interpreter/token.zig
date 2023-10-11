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
    Defer,
    When,
    Inline,
    True,
    False,
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
    Dyn,
    List,
    // Modes
    Mut,
    Move,
    Local,
    Compile,
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
        } else if (std.mem.eql(u8, string, "return")) {
            return Token.Return; 
        } else if (std.mem.eql(u8, string, "def")) {
            return Token.Def; 
        } else if (std.mem.eql(u8, string, "record")) {
            return Token.Record; 
        } else if (std.mem.eql(u8, string, "variant")) {
            return Token.Variant; 
        } else if (std.mem.eql(u8, string, "behaviour")) {
            return Token.Behaviour; 
        } else if (std.mem.eql(u8, string, "module")) {
            return Token.Module; 
        } else if (std.mem.eql(u8, string, "defer")) {
            return Token.Defer; 
        } else if (std.mem.eql(u8, string, "when")) {
            return Token.When; 
        } else if (std.mem.eql(u8, string, "inline")) {
            return Token.Inline; 
        } else if (std.mem.eql(u8, string, "true")) {
            return Token.True; 
        } else if (std.mem.eql(u8, string, "false")) {
            return Token.False; 
        } else if (std.mem.eql(u8, string, "do")) {
            return Token.Do; 
        } else if (std.mem.eql(u8, string, "end")) {
            return Token.End; 
        } else if (std.mem.eql(u8, string, "for")) {
            return Token.For; 
        } else if (std.mem.eql(u8, string, "while")) {
            return Token.While; 
        } else if (std.mem.eql(u8, string, "break")) {
            return Token.Break; 
        } else if (std.mem.eql(u8, string, "continue")) {
            return Token.Continue; 
        } else if (std.mem.eql(u8, string, "match")) {
            return Token.Match; 
        } else if (std.mem.eql(u8, string, "if")) {
            return Token.If; 
        } else if (std.mem.eql(u8, string, "else")) {
            return Token.Else; 
        } else if (std.mem.eql(u8, string, "as")) {
            return Token.As; 
        } else if (std.mem.eql(u8, string, "anytype")) {
            return Token.Anytype; 
        } else if (std.mem.eql(u8, string, "and")) {
            return Token.And; 
        } else if (std.mem.eql(u8, string, "or")) {
            return Token.Or; 
        } else if (std.mem.eql(u8, string, "dyn")) {
            return Token.Dyn; 
        } else if (std.mem.eql(u8, string, "list")) {
            return Token.List; 
        } else if (std.mem.eql(u8, string, "mut")) {
            return Token.Mut; 
        } else if (std.mem.eql(u8, string, "move")) {
            return Token.Move; 
        } else if (std.mem.eql(u8, string, "local")) {
            return Token.Local; 
        } else if (std.mem.eql(u8, string, "comptime")) {
            return Token.Comptime; 
        } else {
            return null;
        }
    }

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

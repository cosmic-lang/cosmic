//!
//!

use std::rc::Rc;

use super::token::{Token, TokenType};
use super::super::utilities::Position;

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

/// Holds on to source code and provides an iterator for 
/// tokenizing the source as needed
pub struct Scanner {
    source: Box<str>,
    read: usize,
    peek: usize,
    char: char,
    file_name: Rc<str>,
    file_pos: Position
}

impl<'a> Scanner {
    /// Returns a new scanner
    pub fn new(file_name: Rc<str>, source: Box<str>) -> Scanner {
        Self {
            char: source.chars().nth(0).unwrap(),
            source,
            read: 0,
            peek: 1,
            file_name,
            file_pos: Position{col: 1, line: 1}
        }
    }

    /// Resets the scanner to the beginning of the file
    pub fn reset(&mut self) {
        self.read = 0;
        self.peek = 1;
        self.char = self.source.chars().nth(0).unwrap();
        self.file_pos = Position{col: 1, line: 1};
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

            self.file_pos.col += 1;

            if self.read > 0 && self.source.chars().nth(self.read - 1) == Some('\n') {
                self.file_pos.col = 1;
                self.file_pos.line += 1;
            }
        }
    }

    /// Returns the next char
    fn peek(&self) -> char {
        return match self.peek >= self.source.len() {
            false => self.source.chars().nth(self.peek).unwrap(),
            true => '\0'
        }
    }

    /// Returns the char at peek + count, skipping whitespace
    fn peek_plus(&self, count: usize) -> char {
        return match self.peek + count >= self.source.len() {
            false => self.source.chars().nth(self.peek + count).unwrap(),
            true => '\0'
        }
    }

    /// Skips whitespace, preserves \n
    fn skip_whitespace(&mut self) {
        match self.char {
            ' ' | '\t' => {
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
    fn read_tag(&mut self, start: usize, mut end: usize) -> Token {
        if is_alphanumeric(self.char) {
            self.advance(1);
            return self.read_tag(start, end + 1)
        }

        if self.char == '?' || self.char == '!' {
            self.advance(1);
            end += 1
        }

        let tt = match TokenType::try_keyword(&self.source[start..end]) {
            Some(keyword) => keyword,
            None => TokenType::Tag(self.source[start..end].into())
        };

        return Token{
            tt,
            pos: Position{
                col: self.file_pos.col - (end - start), 
                line: self.file_pos.line
            },
            file_name: self.file_name.clone()
        }
    }

    /// Reads integers and floats, if '.' is read, switches to read_float to read the decimals
    fn read_number(&mut self, start: usize, end: usize) -> Token {
        if self.char == '.' {
            self.advance(1);
            return self.read_float(start, end + 1)
        } else if is_numeric(self.char) {
            self.advance(1);
            return self.read_number(start, end + 1)
        }

        return Token{
            tt: TokenType::Integer(self.source[start..end].into()),
            pos: Position{
                col: self.file_pos.col - (end - start), 
                line: self.file_pos.line
            },
            file_name: self.file_name.clone()
        }
    }

    /// Read the decimal portion of the float
    fn read_float(&mut self, start: usize, end: usize) -> Token {
        if is_integer(self.char) {
            self.advance(1);
            return self.read_float(start, end + 1)
        }

        return Token{
            tt: TokenType::Float(self.source[start..end].into()),
            pos: Position{
                col: self.file_pos.col - (end - start), 
                line: self.file_pos.line
            },
            file_name: self.file_name.clone()
        }
    }

    /// Returns &str for escape characters, else None
    fn _check_escape(&mut self) -> Option<char> {
        match self.peek() {
            'n' => {
                self.advance(2);
                Some('\n')
            },
            'r' => {
                self.advance(2);
                Some('\r')
            },
            't' => {
                self.advance(2);
                Some('\t')
            },
            '\\' => {
                self.advance(2);
                Some('\\')
            },
            '"' => {
                self.advance(2);
                Some('"')
            },
            ch if is_integer(ch) => {
                self.advance(1);

                Some(' ')
            },
            'u' => {
                self.advance(2);

                Some(' ')
            },
            _ => None
        }
    }

    /// Reads strings
    fn read_string(&mut self, start: usize, end: usize) -> Token {
        if self.char != '"' && self.read < self.source.len() {
            self.advance(1);
            return self.read_string(start, end + 1);
        }

        self.advance(1);

        return Token{
            tt: TokenType::String(self.source[start..end].into()),
            pos: Position{
                col: self.file_pos.col - (end - start), 
                line: self.file_pos.line
            },
            file_name: self.file_name.clone()
        }
    }

    /// Reads multiline strings
    fn read_multistring(&'a mut self, str: &mut Vec<String>, pos: Position, start: usize, end: usize) -> Token {
        if self.read < self.source.len() {
            if self.char != '\n' {
                //if let Some(strg) = self.check_escape() {
                //  str.push(strg);
                //} else {
                self.advance(1);
                return self.read_multistring(str, pos, start, end + 1);
                //}
            } else if self.char == '\n' {
                str.push(self.source[start..end].to_owned());

                self.advance(1);
                self.skip_whitespace();

                // If next line starts with \\, push \n and call recursively to read next line
                if self.char == '\\' && self.peek() == '\\' {
                    str.push("\n".to_owned());
                    self.advance(2);
                    return self.read_multistring(str, pos, self.read, self.read);
                }
            }
        }

        // Concat Vec<&str> into Box<str> and return token
        return Token{
            tt: TokenType::String(str.concat().into()),
            pos,
            file_name: self.file_name.clone()
        }
    }

    /// Reads regex
    fn read_regex(&mut self, start: usize, end: usize) -> Token {
        if self.char != '`' && self.read < self.source.len() {
            self.advance(1);
            return self.read_regex(start, end + 1);
        }

        self.advance(1);

        return Token{
            tt: TokenType::Regex(self.source[start..end].into()),
            pos: Position{
                col: self.file_pos.col - (end - start), 
                line: self.file_pos.line
            },
            file_name: self.file_name.clone()
        }
    }

    /// Checks for compound operators
    fn compound_or_else(&mut self, rules: Vec<(char, TokenType)>, default: TokenType) -> TokenType {
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
    fn compound_three(&mut self, rules: Vec<(char, char, TokenType)>) -> Option<TokenType> {
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

    // Matches self.char and gets the assosiated token
    fn get_next_token(&mut self) -> Token {
        match self.char {
            // Tags and Keywords
            '#' => {
                self.skip_comment();
                self.get_next_token()
            }
            ch if is_alphabetical(ch) => {
                self.read_tag(self.read, self.read)
            },
            // Integers and Floats
            ch if is_integer(ch) => {
                self.read_number(self.read, self.read)
            },
            // Strings
            '"' => {
                self.advance(1);
                self.read_string(self.read, self.read)
            },
            // Multiline Strings
            '\\' if self.peek() == '\\' => {
                self.advance(2);
                self.read_multistring(&mut vec![], self.file_pos, self.read, self.read)
            },
            // Regex
            '`' => {
                self.advance(1);
                self.read_regex(self.read, self.read)
            },
            // Compound operators
            '=' => {
                let tt = self.compound_or_else(vec![
                    ('>', TokenType::FatArrow),
                    ('~', TokenType::PatternMatch),
                    ('=', TokenType::Equal)
                ], TokenType::Assign);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Assign => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            '+' => { 
                let tt = self.compound_or_else(vec![
                    ('+', TokenType::Increment)
                ], TokenType::Plus);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Plus => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            '*' => { 
                let tt = self.compound_or_else(vec![
                    ('*', TokenType::Power)
                ], TokenType::Asterisk);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Asterisk => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            '-' => {
                let tt = self.compound_or_else(vec![
                    ('>', TokenType::Arrow),
                    ('-', TokenType::Decrement)
                ], TokenType::Minus);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Minus => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            '<' => {
                let tt = self.compound_or_else(vec![
                    ('=', TokenType::LesserEq),
                    ('<', TokenType::Lshift)
                ], TokenType::Lesser);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Lesser => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            '>' => {
                let tt = self.compound_or_else(vec![
                    ('=', TokenType::GreaterEq),
                    ('>', TokenType::Rshift)
                ], TokenType::Greater); 

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Greater => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            '!' => {
                let tt = self.compound_or_else(vec![
                    ('=', TokenType::NotEqual),
                    ('~', TokenType::PatternNotMatch)
                ], TokenType::Bang);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Bang => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            ':' => {
                let tt = self.compound_or_else(vec![
                    ('=', TokenType::AssignExp)
                ], TokenType::Colon);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Colon => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            '|' => { 
                let tt = self.compound_or_else(vec![
                    ('>', TokenType::Pipeline)
                ], TokenType::Pipe);

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Pipe => 1,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
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

                let pos = Position{
                    col: self.file_pos.col - match tt {
                        TokenType::Dot => 1,
                        TokenType::RangeInc => 3,
                        _ => 2
                    },
                    line: self.file_pos.line
                };

                Token{tt, pos, file_name: self.file_name.clone()}
            },
            // All else
            ch => {
                let tok = Token{
                    pos: self.file_pos,
                    tt: TokenType::of_char(ch), 
                    file_name: self.file_name.clone()
                };
                
                self.advance(1);
                tok
            }
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;

    /// Scans the next Token from the source
    fn next(&mut self) -> Option<Self::Item> {
        if self.read > self.source.len() {
            return None;
        }

        self.skip_whitespace();

        let token = self.get_next_token();

        return Some(token);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn test_tokens(tokens: &Vec<Token>, expected: &Vec<TokenType>) {
        assert_eq!(tokens.len(), expected.len());

        for (t, e) in tokens.iter().zip(expected.iter()) {
            assert_eq!(t.tt, *e)
        }
    }

    #[test]
    fn operators() {
        let source = r#"+-*/<>!@$%&^=|;:?,."#;
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
            TokenType::Dot,
            TokenType::Eof
        ];

        let mut scanner = Scanner::new("test".into(), source.into());

        let mut tokens = Vec::new();

        loop {
            if let Some(token) = scanner.next() {
                tokens.push(token);
            } else {
                break;
            }
        }

        test_tokens(&tokens, &expected)
    }

    #[test]
    fn numbers() {
        let source = r#"
        # imma comment
        1234
        12.4 # Another Comment
        12.2.4
        "#.trim_start();

        let expected = vec![
            TokenType::Newline,
            TokenType::Integer("1234".into()),
            TokenType::Newline,
            TokenType::Float("12.4".into()),
            TokenType::Newline,
            TokenType::Float("12.2".into()),
            TokenType::Dot,
            TokenType::Integer("4".into()),
            TokenType::Newline,
            TokenType::Eof
        ];

        let mut scanner = Scanner::new("test".into(), source.into());

        let mut tokens = Vec::new();

        loop {
            if let Some(token) = scanner.next() {
                tokens.push(token);
            } else {
                break;
            }
        }

        test_tokens(&tokens, &expected)
    }

    #[test]
    fn strings() {
        let source = r#"
        "12.2.1"
        "hello"
        \\hello, world
        \\!
        "#.trim_start();

        let expected = vec![
            TokenType::String("12.2.1".into()),
            TokenType::Newline,
            TokenType::String("hello".into()),
            TokenType::Newline,
            TokenType::String("hello, world\n!".into()),
            TokenType::Eof
        ];

        let mut scanner = Scanner::new("test".into(), source.into());

        let mut tokens = Vec::new();

        loop {
            if let Some(token) = scanner.next() {
                tokens.push(token);
            } else {
                break;
            }
        }

        test_tokens(&tokens, &expected)
    }

    #[test]
    fn tags_and_keywords() {
        let source = r#"
        hello
        hey?
        yo!
        let
        "#.trim_start();

        let expected = vec![
            TokenType::Tag("hello".into()),
            TokenType::Newline,
            TokenType::Tag("hey?".into()),
            TokenType::Newline,
            TokenType::Tag("yo!".into()),
            TokenType::Newline,
            TokenType::Let,
            TokenType::Newline,
            TokenType::Eof
        ];

        let mut scanner = Scanner::new("test".into(), source.into());

        let mut tokens = Vec::new();

        loop {
            if let Some(token) = scanner.next() {
                tokens.push(token);
            } else {
                break;
            }
        }

        test_tokens(&tokens, &expected)
    }

    #[test]
    fn compound_operators() {
        let source = r#"<==>->!!=..:=...=~!~"#;

        let expected = vec![
            TokenType::LesserEq,
            TokenType::FatArrow,
            TokenType::Arrow,
            TokenType::Bang,
            TokenType::NotEqual,
            TokenType::RangeExc,
            TokenType::AssignExp,
            TokenType::RangeInc,
            TokenType::PatternMatch,
            TokenType::PatternNotMatch,
            TokenType::Eof
        ];

        let mut scanner = Scanner::new("test".into(), source.into());

        let mut tokens = Vec::new();

        loop {
            if let Some(token) = scanner.next() {
                tokens.push(token);
            } else {
                break;
            }
        }

        test_tokens(&tokens, &expected)
    }

    #[test]
    fn assignment() {
        let source = r#"
        let x: int = 12
        const y: regex = `rex(lang|xer)`
        let z: string = "rexlang"
        z =~ y
        "#.trim_start();

        let expected = vec![
            TokenType::Let,
            TokenType::Tag("x".into()),
            TokenType::Colon,
            TokenType::Tag("int".into()),
            TokenType::Assign,
            TokenType::Integer("12".into()),
            TokenType::Newline,
            TokenType::Const,
            TokenType::Tag("y".into()),
            TokenType::Colon,
            TokenType::Tag("regex".into()),
            TokenType::Assign,
            TokenType::Regex("rex(lang|xer)".into()),
            TokenType::Newline,
            TokenType::Let,
            TokenType::Tag("z".into()),
            TokenType::Colon,
            TokenType::Tag("string".into()),
            TokenType::Assign,
            TokenType::String("rexlang".into()),
            TokenType::Newline,
            TokenType::Tag("z".into()),
            TokenType::PatternMatch,
            TokenType::Tag("y".into()),
            TokenType::Newline,
            TokenType::Eof
        ];

        let mut scanner = Scanner::new("test".into(), source.into());

        let mut tokens = Vec::new();

        loop {
            if let Some(token) = scanner.next() {
                tokens.push(token);
            } else {
                break;
            }
        }

        test_tokens(&tokens, &expected)
    }
}

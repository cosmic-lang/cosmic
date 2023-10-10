//!
//!

const std = @import("std");

const Token = @import("token.zig").Token;

pub const Scanner = struct {
    const Self = @This();

    source: []u8,
    pos: usize,
    read: usize,
    char: u8,
    tokens: std.ArrayList(Token),

    pub fn init(allocator: *const std.mem.Allocator, source: []u8) Self {
        return Self{
            .source = source,
            .pos = 0,
            .read = 1,
            .char = source[0],
            .tokens = std.ArrayList(Token).init(allocator.*),
        };
    }

    pub fn deinit(self: *Self) void {
        self.tokens.deinit(); 
    }

    fn advance(self: *Self) ?void {
        if (self.pos + 1 > self.source.len) {
            return null;
        }

        self.pos = self.read;
        self.read += 1;
        self.char = self.source[self.pos];
    }

    fn peek(self: *Self) u8 {
        return self.source[self.read];
    }

    fn is_alphabetic(self: *Self) bool {
        switch (self.char) {
            'a'...'z',
            'A'...'Z',
            '_' => return true
        }
    }
    
    fn is_numeric(self: *Self) bool {
        switch (self.char) {
           '0'...'9',
           '.' => return true
        }
    }
    
    fn is_alphanumeric(self: *Self) bool {
        if (self.numeric() and self.is_alphabetic()) {
            return true;
        }
    }

    pub fn next_token(self: *Self) Token {
        while (self.advance != null) {
            switch (self.char) {
              else => return Token{.tag = self.source}
            }
        }
    }
};

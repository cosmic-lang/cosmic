//!
//!

const std = @import("std");

const Token = @import("token.zig").Token;

pub const Scanner = struct {
    const Self = @This();

    source: []u8,
    current_char: usize,
    peek_char: usize,
    char: u8,
    tokens: std.ArrayList(Token),

    pub fn init(allocator: *const std.mem.Allocator, source: []u8) Self {
        return Self{
            .source = source,
            .tokens = std.ArrayList(Token).init(allocator.*),
        };
    }

    pub fn deinit(self: *Self) void {
        self.tokens.deinit(); 
    }

    fn advance(self: *Self) ?void {
        if (self.current_char + 1 > self.source.len) {
            return null;
        }

        self.current_char += 1;
        self.char = self.source[self.current_char];
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

    pub fn scan(self: *Self) !std.ArrayList(Token) {
        while (self.advance != null) {
            switch (self.char) {
                 
            }
        }

        try self.tokens.append(Token{.identifier = self.source});
        return self.tokens;
    }
};

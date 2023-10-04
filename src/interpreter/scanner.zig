//!
//!

const std = @import("std");

const Token = @import("token.zig").Token;

pub const Scanner = struct {
  const Self = @This();

  source: []u8,
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

  pub fn scan(self: *Self) !std.ArrayList(Token) {
    try self.tokens.append(Token{.identifier = self.source});
    return self.tokens;
  }
};

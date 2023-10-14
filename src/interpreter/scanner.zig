//!
//!

const std = @import("std");

const Token = @import("token.zig").Token;

fn is_alphabetic(char: u8) bool {
  switch (char) {
    'A'...'Z',
    'a'...'z',
    '_' => return true,
    else => return false
  }
}

fn is_numeric(char: u8) bool {
  switch (char) {
    '0'...'9' => return true,
    else => return false
  }
}

fn is_alphanumeric(char: u8) bool {
  if (is_numeric(char) or is_alphabetic(char)) {
    return true;
  }
  return false;
}


pub const Scanner = struct {
  const Self = @This();

  source: []const u8,
  pos: usize,
  read: usize,
  char: u8,

  keywords: std.StringHashMap(Token),

  pub fn new(source: []const u8) !Self {
    return Self{
      .source = source,
      .pos = 0,
      .read = 1,
      .char = source[0],
      .keywords = undefined,
    };
  }

  pub fn init(self: *Self, allocator: std.mem.Allocator) !void {
    self.keywords = std.StringHashMap(Token).init(allocator);
    // Setup keyword hash
    try self.keywords.put("const", .Const);
    try self.keywords.put("let", .Let);
    try self.keywords.put("return", .Return);
    try self.keywords.put("fn", .Fn);
    try self.keywords.put("record", .Record);
    try self.keywords.put("enum", .Enum);
    try self.keywords.put("trait", .Trait);
    try self.keywords.put("module", .Module);
    try self.keywords.put("defer", .Defer);
    try self.keywords.put("when", .When);
    try self.keywords.put("inline", .Inline);
    try self.keywords.put("true", .True);
    try self.keywords.put("false", .False);
    try self.keywords.put("for", .For);
    try self.keywords.put("while", .While);
    try self.keywords.put("break", .Break);
    try self.keywords.put("continue", .Continue);
    try self.keywords.put("match", .Match);
    try self.keywords.put("if", .If);
    try self.keywords.put("else", .Else);
    try self.keywords.put("as", .As);
    try self.keywords.put("anytype", .Anytype);
    try self.keywords.put("and", .And);
    try self.keywords.put("or", .Or);
    try self.keywords.put("dyn", .Dyn);    
    try self.keywords.put("mut", .Mutable);
    try self.keywords.put("mov", .Move);
    try self.keywords.put("loc", .Local);
    try self.keywords.put("comptime", .Comptime);
  }

  pub fn deinit(self: *Self) void {
    self.keywords.deinit();
  }

  fn advance(self: *Self) void {
    if (self.read >= self.source.len) {
      self.char = '\x00';
    } else {
      self.char = self.source[self.read];
    }

    self.pos = self.read;
    self.read = self.read + 1;
  }

  fn peek(self: *Self) ?u8 {
    if (self.read >= self.source.len) {
      return null;
    }

    return self.source[self.read];
  }

  fn read_tag(self: *Self) []const u8 {
    const start = self.pos;
    var end: usize = self.source.len;

    while (self.peek()) |char| {
      if (!is_alphanumeric(char)) {
        end = self.read;
        self.advance();
        break;
      }

      self.advance();
    } else {
      self.advance();
    }

    return self.source[start..end];
  }

  fn read_number(self: *Self) []const u8 {
    const start = self.pos;
    var end: usize = self.source.len;

    while (self.peek()) |char| {
      if (!is_numeric(char)) {
        end = self.read;
        self.advance();
        break;
      }

      self.advance();
    } else {
      self.advance();
    }

    return self.source[start..end];
  }

  fn try_keyword(self: *Self, string: []const u8) ?Token {
    return self.keywords.get(string);

  }

  fn skip_whitespace(self: *Self) void {
    switch (self.char) {
      ' ', '\r', '\t' => {
        self.advance();
        self.skip_whitespace();
      },
      else => {}
    }
  }

  pub fn next_token(self: *Self) !?Token {
    if (self.read > self.source.len + 1) {
      return null;
    }

    self.skip_whitespace();

    if (is_alphabetic(self.char)) {
      var tag = self.read_tag();

      if (self.try_keyword(tag)) |keyword| {
        return keyword;
      } else {
        return Token{.tag = tag};
      }
    } else if (is_numeric(self.char)) {
      var number_str = self.read_number();
      const number = try std.fmt.parseInt(i32, number_str, 10);

      return Token{.integer = number};
    } else {
      const char = self.char;
      self.advance();

      return Token.of_char(char);
    }
  }
};

fn test_tokens(expected: []const Token, got: []const Token) !void {
  for (expected, got) |e, g| {
    std.debug.assert(@intFromEnum(e) == @intFromEnum(g));

    switch (e) {
      .tag => |etag| {
        switch (g) {
          .tag => |gtag| {
            try std.testing.expect(std.mem.eql(u8, etag, gtag));
          },
          else => try std.testing.expect(false)
        }
      },
      .integer => |eint| {
        switch (g) {
          .integer => |gint| {
            try std.testing.expect(eint == gint);
          },
          else => try std.testing.expect(false)
        }
      },
      else => {}
    }
  }
}

test "scanner" {
  // Setup arena allocator
  const source = 
    \\const x = 12
    \\let y = 13
    ;

  const expected = [_]Token{
    Token.Const,
    Token{.tag = "x"},
    Token.assign,
    Token{.integer = 12},
    Token.newline,
    Token.Let,
    Token{.tag = "y"},
    Token.assign,
    Token{.integer = 13},
    Token.eof
  };

  // Scan file
  var scanner = try Scanner.new(source);
  try scanner.init(std.testing.allocator);
  defer scanner.deinit();

  var tokens = std.ArrayList(Token).init(std.testing.allocator);
  defer tokens.deinit();

  while (try scanner.next_token()) |token| {
    try tokens.append(token);
  }

  try test_tokens(expected[0..], tokens.items);
  // Print tokens
  //std.debug.print("\n", .{});
  //for (tokens.items) |token| {
  //  switch (token) {
  //    Token.tag => |ident| std.debug.print("{s} ", .{ident}),
  //    Token.integer => |int| std.debug.print("{d} ", .{int}),
  //    Token.Const => std.debug.print("const ", .{}),
  //    Token.Let => std.debug.print("let ", .{}),
  //    Token.assign => std.debug.print("= ", .{}),
  //    Token.newline => std.debug.print("\n", .{}),
  //    Token.eof => std.debug.print("eof", .{}),
  //    else => {}
  //  }
  //}

  //std.debug.print("\n", .{});
}

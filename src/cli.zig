//!
//!

const std = @import("std");
const clap = @import("clap");

//const Args = struct {
//  comptime prompt: ?[]const u8 = null
//};

pub const Repl = struct {
  const Self = @This();

  prompt: []const u8,

  pub fn init(prompt: []const u8, _: anytype) Self {
    return Self {
      .prompt = prompt
    };
  }

  pub fn deinit(_: *Self) void {

  }

  pub fn run(self: *Self) !void {
    std.debug.print("{s} ", .{self.prompt});

    //self.run();
  }
};

pub fn help(
  comptime logo: []const u8, 
  comptime header: []const u8, 
  comptime version: []const u8, 
  params: anytype
) !void {
  std.debug.print(logo ++ "\n", .{});
  std.debug.print(header ++ "\n", .{version});
  std.debug.print("Subcommands:\n", .{});

  return clap.help(
    std.io.getStdErr().writer(), 
    clap.Help, 
    params, 
    .{}
  );
}

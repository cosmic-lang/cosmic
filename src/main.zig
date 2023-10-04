const std = @import("std");
const cli = @import("cli.zig");
const script = @import("script.zig");
const clap = @import("clap");

const MAX_BYTES = 1024^2;
const VERSION = "0.0.1";
const EXTENSION = ".tali";
const PROMPT = "talisman>";
const LOGO = 
  \\
  ;
const HEADER =
  \\Welcome to the Talisman programming language!
  \\
  ++ "  version {s}\n" ++
  \\
  \\  usage   : talisman [subcommand] <arg>
  \\  example : talisman --script foo.be
  \\
  ;

pub fn main() !void {
  // Setup arena allocator
  var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
  const allocator = arena.allocator();
  defer arena.deinit();

  // Setup clap parameters and diagnostics
  const params = comptime clap.parseParamsComptime(
    \\-h, --help          Displays help and exits.
    \\-v, --version       Displays version and exits.
    \\-s, --script <str>  Runs script file at relative path.
    \\-r, --repl          Starts interactive repl.
    \\
  );
  var diag = clap.Diagnostic{};

  // Parse args
  const res = clap.parse(clap.Help, &params, clap.parsers.default, .{.diagnostic = &diag}) 
  catch |err| {
    diag.report(std.io.getStdErr().writer(), err) catch {};
    return;
  };
  defer res.deinit();

  // Check arguments
  if (res.args.help != 0) {
    // Show help
    return cli.help(LOGO, HEADER, VERSION, &params);
  } else if (res.args.version != 0) {
    std.debug.print("talisman {s}", .{VERSION});
  } else if (res.args.repl != 0) {
    // Setup repl
    var repl = cli.Repl.init(PROMPT, .{});
    defer repl.deinit();
    // Start repl
    try repl.run();
  } else if (res.args.script) |path| {
    // Interpret script
    try script.run(&allocator, path, EXTENSION, MAX_BYTES);
  } else {
    // Show help
    return cli.help(LOGO, HEADER, VERSION, &params);
  }
}

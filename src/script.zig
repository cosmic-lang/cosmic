//!
//!

const std = @import("std");

const Token = @import("interpreter/token.zig").Token;
const Scanner = @import("interpreter/scanner.zig").Scanner;


pub fn run(allocator: std.mem.Allocator, path: []const u8, ext: []const u8, max_bytes: usize) !void {
    // Check for valid file extension, return if invalid
    const extension = std.fs.path.extension(path);
    if (!std.mem.eql(u8, extension, ext)) {
        if (extension.len == 0) {
            std.debug.print("Missing file extension, expected: {s}", .{ext});
        } else {
            std.debug.print("Invalid file extension: {s}, expected: {s}", .{extension, ext});
        }
        return;
    }

    // Open file
    const file = std.fs.cwd().openFile(path, .{}) catch {
        std.debug.print("File: {s} not found\n", .{path});
        return;
    };
    defer file.close();

    // Read source from file
    const source = try file.reader().readAllAlloc(allocator, max_bytes);

    // Scan file
    var scanner = try Scanner.new(source);
    try scanner.init(allocator);
    defer scanner.deinit();
}

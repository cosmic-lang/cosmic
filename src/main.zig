const std = @import("std");

const clap = @import("clap");

const cli = @import("cli.zig");
const script = @import("script.zig");

const MAX_BYTES = std.math.maxInt(u32);
const VERSION = "0.0.1";
const EXTENSION = ".ruka";
const PROMPT = "Ruka>";
const HEADER =
    \\Welcome to the Ruka programming language!
    \\
    ++ "  version {s}\n" ++
    \\
    \\  usage   : ruka [subcommand] <arg>
    \\  example : ruka --script foo.ruka
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
        \\-c, --compile <str> Compiles file at relative path.
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
        return cli.help(HEADER, VERSION, &params);
    } else if (res.args.version != 0) {
        std.debug.print("Ruka {s}", .{VERSION});
    } else if (res.args.repl != 0) {
        // Setup repl
        var repl = cli.Repl.init(PROMPT, .{});
        defer repl.deinit();
        // Start repl
        try repl.run();
    } else if (res.args.compile) |path| {
        // Compile file
        try script.run(allocator, path, EXTENSION, MAX_BYTES);
    } else if (res.args.script) |path| {
        // Interpret script
        try script.run(allocator, path, EXTENSION, MAX_BYTES);
    } else {
        // Show help
        return cli.help(HEADER, VERSION, &params);
    }
}

//const llvm = @import("llvm");
//
//const target = llvm.target;
//const types = llvm.types;
//const core = llvm.core;
//
//pub fn main() !void {
//    _ = target.LLVMInitializeNativeTarget();
//    _ = target.LLVMInitializeNativeAsmPrinter();
//    _ = target.LLVMInitializeNativeAsmParser();
//
//    var module: types.LLVMModuleRef = core.LLVMModuleCreateWithName("sum_module");
//    var params: [2]types.LLVMTypeRef = [_]types.LLVMTypeRef{
//        core.LLVMInt32Type(),
//        core.LLVMInt32Type()
//    };
//
//    // Create a function that computes the sum of two integers
//    const func_type: types.LLVMTypeRef = core.LLVMFunctionType(core.LLVMInt32Type(), &params, 2, 0);
//    const sum_func: types.LLVMValueRef = core.LLVMAddFunction(module, "sum", func_type);
//    const entry: types.LLVMBasicBlockRef = core.LLVMAppendBasicBlock(sum_func, "entry");
//    const builder: types.LLVMBuilderRef = core.LLVMCreateBuilder();
//    core.LLVMPositionBuilderAtEnd(builder, entry);
//    const arg1: types.LLVMValueRef = core.LLVMGetParam(sum_func, 0);
//    const arg2: types.LLVMValueRef = core.LLVMGetParam(sum_func, 1);
//    const sum: types.LLVMValueRef = core.LLVMBuildAdd(builder, arg1, arg2, "sum");
//    _ = core.LLVMBuildRet(builder, sum);
//
//    // Dump the LLVM module to stdout
//    core.LLVMDumpModule(module);
//
//    // Clean up LLVM resources
//    core.LLVMDisposeBuilder(builder);
//    core.LLVMDisposeModule(module);
//    core.LLVMShutdown();
//}

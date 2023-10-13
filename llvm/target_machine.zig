const LLVMtype = @import("types.zig");

pub extern fn LLVMGetFirstTarget() LLVMtype.LLVMTargetRef;
pub extern fn LLVMGetNextTarget(T: LLVMtype.LLVMTargetRef) LLVMtype.LLVMTargetRef;
pub extern fn LLVMGetTargetFromName(Name: [*c]const u8) LLVMtype.LLVMTargetRef;
pub extern fn LLVMGetTargetFromTriple(Triple: [*c]const u8, T: [*c]LLVMtype.LLVMTargetRef, ErrorMessage: [*c][*c]u8) LLVMtype.LLVMBool;
pub extern fn LLVMGetTargetName(T: LLVMtype.LLVMTargetRef) [*c]const u8;
pub extern fn LLVMGetTargetDescription(T: LLVMtype.LLVMTargetRef) [*c]const u8;
pub extern fn LLVMTargetHasJIT(T: LLVMtype.LLVMTargetRef) LLVMtype.LLVMBool;
pub extern fn LLVMTargetHasTargetMachine(T: LLVMtype.LLVMTargetRef) LLVMtype.LLVMBool;
pub extern fn LLVMTargetHasAsmBackend(T: LLVMtype.LLVMTargetRef) LLVMtype.LLVMBool;
pub extern fn LLVMCreateTargetMachine(T: LLVMtype.LLVMTargetRef, Triple: [*c]const u8, CPU: [*c]const u8, Features: [*c]const u8, Level: LLVMtype.LLVMCodeGenOptLevel, Reloc: LLVMtype.LLVMRelocMode, CodeModel: LLVMtype.LLVMCodeModel) LLVMtype.LLVMTargetMachineRef;
pub extern fn LLVMDisposeTargetMachine(T: LLVMtype.LLVMTargetMachineRef) void;
pub extern fn LLVMGetTargetMachineTarget(T: LLVMtype.LLVMTargetMachineRef) LLVMtype.LLVMTargetRef;
pub extern fn LLVMGetTargetMachineTriple(T: LLVMtype.LLVMTargetMachineRef) [*c]u8;
pub extern fn LLVMGetTargetMachineCPU(T: LLVMtype.LLVMTargetMachineRef) [*c]u8;
pub extern fn LLVMGetTargetMachineFeatureString(T: LLVMtype.LLVMTargetMachineRef) [*c]u8;
pub extern fn LLVMCreateTargetDataLayout(T: LLVMtype.LLVMTargetMachineRef) LLVMtype.LLVMTargetDataRef;
pub extern fn LLVMSetTargetMachineAsmVerbosity(T: LLVMtype.LLVMTargetMachineRef, VerboseAsm: LLVMtype.LLVMBool) void;
pub extern fn LLVMTargetMachineEmitToFile(T: LLVMtype.LLVMTargetMachineRef, M: LLVMtype.LLVMModuleRef, Filename: [*c]const u8, codegen: LLVMtype.LLVMCodeGenFileType, ErrorMessage: [*c][*c]u8) LLVMtype.LLVMBool;
pub extern fn LLVMTargetMachineEmitToMemoryBuffer(T: LLVMtype.LLVMTargetMachineRef, M: LLVMtype.LLVMModuleRef, codegen: LLVMtype.LLVMCodeGenFileType, ErrorMessage: [*c][*c]u8, OutMemBuf: [*c]LLVMtype.LLVMMemoryBufferRef) LLVMtype.LLVMBool;
pub extern fn LLVMGetDefaultTargetTriple() [*c]u8;
pub extern fn LLVMNormalizeTargetTriple(triple: [*c]const u8) [*c]u8;
pub extern fn LLVMGetHostCPUName() [*c]u8;
pub extern fn LLVMGetHostCPUFeatures() [*c]u8;
pub extern fn LLVMAddAnalysisPasses(T: LLVMtype.LLVMTargetMachineRef, PM: LLVMtype.LLVMPassManagerRef) void;

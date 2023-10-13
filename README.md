# `Ruka`
A general purpose, statically typed, programming language.

# ! In Early Development
Implementation of `Ruka` has just started, so it is not usable yet.  
Development is following three main stages:
- Stage 1: Scanning/Parsing [in progress]
- Stage 2: Simple Interpreter
- Stage 3: Compilation, either using LLVM or to ASM directly

# License
`Ruka` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

# Features
- Strong static typing
- Type inference
- First class `Types`, `Errors`, `Functions`, and `Modules`
- Shared functionality with interfaces called `Traits`
- Pattern matching
- Meta-programming with compile time execution
- Type reflection
- Automatic memory management by default
    - Variables are stack or GC allocated based on usage
    - Can manually manage heap memory with allocators
- Borrows for when safety is a priority, Pointers for when it's not

# Stretch Goal: Silver
`Ruka` may have an extension called `Silver`, which integrates `Ruka` and HDL for simple FPGA development

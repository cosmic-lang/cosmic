# `Ruka`
A general purpose, statically typed, programming language.

# ! In Early Development
`Ruka` is currently in the design stage, so the language has yet to be implemented, and everything is subject to drastic change.

# License
`Ruka` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

# Features
- Strong static typing
- Type inference
- First class Types, Errors, Functions, and Modules
- Shared functionality with interfaces called `Behaviours`
- Pattern matching
- Meta-programming with compile time execution
- Type reflection
- Green threads called `Strings`
- Reactivity with `Signals`
- Automatic memory management by default
  - Variables are stack or GC allocated based on usage
  - Can manually manage heap memory with allocators
- Per project language customization
  - Disable GC
  - Type inference customization
- References for when safety is a priority, Pointers for when it's not

# Stretch Goal: Silver
`Ruka` has an extension called `Silver`, which integrated `Ruka` and HDL for simple FPGA development

# Example
```elixir
const List = (@type: typeid): moduleid do
  return module {
    const node = struct {
      next: ?Self,
      data: type
    }

    pub const t = struct {
      head: ?node,
      size: uint
    }

    def insert<exc l: &t> = (value: type) {
      if (l.size == 0) {
        l.head = node{
          next: null,
          data: value
        }
      } else {
        let tmp = l.head

        l.head = node{
          next: tmp,
          data: value
        }
      }

      l.size++ 
    }
  }
end

let names = List(string).t{}

names.insert("foo")
names.insert("bar")
names.insert("foobar")
```

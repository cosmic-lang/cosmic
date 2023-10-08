# `Ruka`
`Ruka` is a general purpose, statically typed, programming language.

# ! In Early Development
`Ruka` is currently in the design stage, so the language has yet to be implemented, and everything is subject to drastic change.

# License
`Ruka` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

# Features

## Types and Inference
Types in `Ruka` are static, and are inferred by default, except for a few cases. They can also be specified if desired.
```elixir
let x
x = 12 # x will be inferred as &int

let name = "Ruka" # name will be inferred as &string

let titles: [dyn]string # titles is specified to be a dyn array of strings
```

## Memory Management
In `Ruka` bindings are stack allocated by default. Memory can be allocated on the heap manually if desired
- Manual management:
  - Using an allocator, you can manage memory manually, which will return a raw pointer to the memory which must be freed before the program }s
- Variable specified to be non-reference types will be stack allocated
```elixir

let name: &string = "hello" # GC allocated, will be freed after the reference goes out of scope

let names: *[5]string = std.allocator.new([5]string) # Allocates an array and returns a raw pointer to it
defer std.allocator.free(names) # Manual memory must be freed

let name: string = "hello" # Specifying a non-reference type will be stack allocated
```

## Bindings are initialized to zero
In `Ruka`, bindings are initialized to default values depending on the type, `0` for numbers, `""` for strings, etc.

## Methods and Receivers
`Method` definition in `Ruka` is done using `receivers` which specify which type the method is a part of, allowing for adding
functionality to any type, even those foreign to your project.
```elixir
const Result = enum{
  Ok(int),
  Err(string)
}

# The receiver follows the method identifier,
# and is specified as a name and type surrounded by <>
const unwrap<r: &Result> = (): int {
  return match r {
    | .Ok(value) do: value,
    | .Err(msg) { 
      std.error.log(msg)
      0
    }
  }
}

# The method can then be called on a instance of Result
let value = someResult.unwrap()

```

## Errors as Values
```elixir

```

## Behaviours
In `Ruka` you use `Go` style interfaces, called `behaviours`, when you want shared functionality between types.
```elixir
const MMIODevice = behaviour{
  read: fn <&>(address: u32) -> u8,
  write: fn <exc&>(address: u32, value: u8) -> ()
}
```

`Behaviours` are implemented for types implicitly like in `Golang`. If a type has matching methods to the ones declared in
the `behaviour`, then it implements the behaviour.
```elixir
const Ram = struct{
  memory: [1024*64]u8
}

# After these two function definitons, Ram implements MMIODevice
const read<r: &Ram> = (address: u32): u8 {
  return r.memory[address]
}

const write<exc r: &Ram> = (address: u32, value: u32) {
  r.memory[address] = value
}
```

Function parameters can then have `behaviours` specified instead of types.
```elixir
const load = (device: &MMIODevice, program: []u8) {
  let len = program.len

  for program, 0..<len |byte, i| {
    device.write(i, byte)
  }
}

let ram = Ram{}
let program: [100]u8

load(&ram, program[..]) # [..] creates a slice covering the entire array
```

## Compile Time Execution
`Ruka` features compile time code execution like in `Zig`. This combined with types as values
is how generic in `Ruka` work.
```elixir
# @ signifies a parameter which must be known at compile time
# typeid is the type of types, i.e. int, string, *u8 
const List = (@t: typeid): typeid {
  const Node = struct{
    next: *Node,
    data: t
  }

  return struct{
    head: *Node,
    size: uint
  }
}

# List(string) returns the new type
# empty {} are used to create an instance of the type with default values
let names = List(string){}
```

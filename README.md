# `Ruka`
`Ruka` is a general purpose, statically typed, programming language.

# ! In Early Development
`Ruka` is currently in the design stage, so the language has yet to be implemented, and everything is subject to drastic change.

# Features

## Types and Inference
Types in `Ruka` are static, and are inferred by default, except for a few cases. They can also be specified if desired.
```elixir
var x
x = 12 # x will be inferred as int

var name = "Ruka" # name will be inferred as string

var titles: [dyn]string # titles is specified to be a dyn array of strings
```

## Bindings are initialized to zero
In `Ruka`, bindings are initialized to default values depending on the type, `0` for numbers, `""` for strings, etc.

## Receivers
`Method` definition in `Ruka` is done using `receivers` which specify which type the method is a part of, allowing for adding
functionality to any type, even those foreign to your project
```elixir
const Result = enum{
  Ok(int),
  Err(string)
}

# The receiver follows the method identifier,
# and is specified as a name and type surrounded by parenthesis
const unwrap(self: &Result) = (): int do
  return match self {
  | .Ok(value) do: value,
  | .Err(msg) do 
    std/error.log(msg)
    0
  end
  }
end

# The method can then be called on a instance of Result
let value = someResult.unwrap()

```

## Errors as Values
```elixir

```

## Behaviours
In `Ruka` you use `Golang` style interfaces, called `behaviours`, when you want shared functionality between types.
```elixir
const MMIODevice = behaviour{
  read: & fn (address: u32) -> u8,
  write: &mut fn (address: u32, value: u8) -> ()
}
```

`Behaviours` are implemented for types implicitly like in `Golang`. If a type has matching methods to the ones declared in
the `behaviour`, then it implements the behaviour.
```elixir
const Ram = struct{
  memory: [1024*64]u8
}

# After these two function definitons, Ram implements MMIODevice
const read(self: &Ram) = (address): u8 do
  return self.memory[address]
end

const write(self: &mut Ram) = (address, value) do
  self.memory[address] = value
end
```

Function parameters can then have `behaviours` specified instead of types.
```elixir
const loadProgram = (ram: &mut Ram, program: []u8) do
  let len = program.len

  for program, 0..<len |byte, i| do
    ram.write(i, byte)
  end
end
```

## Compile time execution
`Ruka` features compile time code execution like in `Ziglang`.
```elixir
# @ signifies a parameter which must be known at compile time
# typeid is the type of types, i.e. int, string, *u8 
const List = (@t: typeid): typeid do
  const Node = struct{
    next: *Node,
    data: t
  }

  return struct{
    head: *Node,
    size: uint
  }
end

# List(string) returns the new type
# empty {} are used to create an instance of the type with default values
var names = List(string){}
```

# License
`Ruka` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

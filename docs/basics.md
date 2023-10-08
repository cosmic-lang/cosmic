# Language Syntax

## Comments
```
# A single-line comment
```

## Bindings
Bindings in `Ruka` follow the form of:  
<pre>
  kind ident [: type] [= expression];
</pre>
with the parts surrounded by [] being optional.  

## Binding Declaration and Initialization
There are three kinds of bindings:

- `const`  
```elixir
# Constants can only be assigned to at compile time
# and so must be a literal or result of a compile time
# expression and must be initialized when declared.
const msg = "Hello, world!"
```

- `let`  
```elixir
# A runtime variable
let year = 2023
year = 2024
```

`Ruka` supports multiple assignment
```elixir
let x = 12
let y = 31
x, y = y, x # swaps bindings with no need for temporary bindings
```

Assignment in `Ruka` can also be done as an expression using ":=", which returns the rhs value
```elixir
let boolean = false
# Assignment expression
while boolean := someFunc() { # Will loop until someFunc returns false 
  std.fmt.printf("{}", boolean)
}
```

Bindings of the same type can be grouped together.
``` elixir
# Var and let bindings still don't need to be initialized right away
let (
  x = 72,
  y = 12
)

```

By default, bindings are allocated by the GC, therefore values are normally & types. But stack allocation
can be done by specifying a non-reference type. 
```elixir
# This will be GC allocated
let x = 12 # &int
# This will be stack allocated
let x: int = 12
```

## Type specification basics

When declaring bindings, types are usually inferred based on later usage of the binding, 
but types can be specified if desired.

<pre>
  kind ident [: type] [= expression];
</pre>

If the binding is not initialized,
then a type specification must be added.
Again, by default bindings are allocated by the GC, therefore values are normally & types.
```elixir
  let x = 83
  let x: &int = 83

  let name: string
```

## Memory Management
In `Ruka` memory is GC allocated by default. Memory can be allocated manually using an allocator
- Manual management:
  - Using an allocator, you can manage memory manually, which will return a pointer to the memory which must be freed before the program }s
- Stack allocation:
  - Variables can be allocated on the stack using the loc mode, which can be inferred
```elixir
let loc x = 12 # Stack allocated, lives until enclosing scope }s
let x: int

let name: &int = 12 # GC allocated, will be freed after the reference goes out of scope

let names: *[5]string = std.allocator.new([5]string) # Allocates an array and returns a pointer to it
defer std.allocator.free(names) # Manual memory must be freed
```

Important note here. GC values are stored as reference, but are dereferenced automatically, so
if a function expects a reference, it still must be passed with & which prevents the dereferencing.
```elixir
let name: &string = "hello" 

const greet = (name: &string) {...}
greet(name) # Error, type mismatch: &string expected, string received
greet(&name)

const greet2 = (name: string) {...}
greet2(name) # Value is copied/Passed by value
```

## Basic Primitive Types
Here is a list of `Ruka`'s primitive types:
- `int`    
  - 12, architecture dependent size
- `i#`     
  - \# bit signed integer i.e. i16
- `uint` 
  - 12, architecture dependent size
- `u#`     
  - \# bit unsigned integer i.e. u8
- `float`  
  - 12.2, architecture dependent size
- `f#`     
  - \# bit float i.e. f32
- `byte`   
  - 'a' or 0xfd
- `string`
  - "Hello, world!"
- `bool` 
  - true, false
- `void` 
  - also (), represents nothing.
- `typeid` 
  - i32, int, char, MyStructure. Types are values in `Ruka`
- `range` 
  - 0..=10, 5..<15
- `tag`   
  - :quick :skip
  - Polymorphic enumerations, i.e. don't need to be part of a type. 
  - Also used for identifiers, when used for identifiers the ":" can be ommited.
  - When used for map keys, the ":" is moved to the rhs
  - When types are specified for bindings, the ":" is moved to the rhs

## Primitive Data Collections
`Ruka` has a few primitive data collections for you to use:
- `Array`
```elixir
# Arrays are static, their sizes cannot change and must be known at compile time
let arr = [1, 2, 3, 4, 5]
let num = arr[2]
std.testing.assert(num == 3)
```

- `Dynamic Array`
```elixir
# Can change size
let arr = [|1, 2, 3|]
let num = arr[1]
std.testing.assert(num == 2)
```

- `List`
```elixir
# Can change size
let list = {|1, 2, 3|}
let num = list[1]
std.testing.assert(num == 2)
```

- `Map`
```elixir
# Can change size
let atomic_mass = %{
  beryllium: 9.1022,
  carbon: 15.999
}

let atomic_mass = %{
  "beryllium" => 9.1022,
  "carbon" => 15.999
}

let carbon_mass = atomic_mass[:carbon]
std.testing.assert(carbon_mass == 15.999) # For floats == only compares the whole number
```

- `Tuple`  

Tuples can be indexed, or destructured using pattern matching. The $len() function can be used to assess the length of a tuple
```elixir
let pos = {10, 15}

std.testing.assert($len(pos) == 2)

let {x, y} = {pos[0], pos[1]}
let x, y = pos # The lhs braces are not required

std.testing.assert(x == 10 && y == 15)
```

## Blocks
Single-line blocks are written using do:
```elixir
do: let x = 83
```

Multi-line blocks are enclosed using braces: {} 
```elixir
{
  let x = 83
}
```

## Conditionals
```elixir
if condition {

} else if another_condition {

} else {

}
```

## Loops
`Ruka` has two looping constructs, range-based for loops, and while loops.
```elixir
for iterable |i| {

}

while condition {

}
```

## Function Basics
All functions in `Ruka` are anonymous, so function definition involves storing a function literal in a binding.
Important note, functions are not closures therefore they can only access their parameters or any locally defined
variables.

Anonymous function creation follows the form of:
<pre>
  ([mode] parameter [: type]) [: return type] body;
</pre>
the body is a block

Function definition follows the form of:
<pre>
  kind ident [: type] = anonymous fn;
</pre>

A single-line body function
```elixir
const hello = () do: return "Hello, world!"
```
values must be returned explicitly


A multi line body.
```elixir
const add = (x, y) do
  return x + y
end

or

const add = (x, y) {
  return x + y
}
```

## Function type specification
```elixir
# Functions that take no parameters have empty "()" before the arrow.
# Void returns can be specified in two ways.
# The return type must always be specified in type specifications.
const foo: fn () -> ()
const bar: fn void -> void

# Types can be specified for multiple parameters at a time.
const add = (x, y: int): int {
  return x + y
}

const add_three = (x, y, z: int): int do: return x + y + z
```

## Modes
Parameters can have constraints on them, called modes. Reference types can only be mutated
in the scope they are defined in. Values passed to functions by reference
cannot be mutated, unless they are passed in the unique or exclusive modes
- Reference types
  - `uni` unique mode, ownership of reference is moved into function
  - `exc` exclusive mode, only one active reference to value so safe to mutate
- All types
  - `@` compile time mode
```elixir
let x, y = 12, 11

const use = (uni x: &int) {}

const add = (x, y: &int) {
  use(&x)
  return x + y # Error x used uniquely so cannot be used twice
}

let name = "foo"

const rename = (exc name: &string) {
  name = "bar"
}

rename(&name)
name # "bar"

```

## Creating new types
- `Struct`  

All structs are anonymous. Members can be accessed with the `.` operator. Members can also be accessed by indexing with a tag, provided the tag is known at compile time.
```elixir
# Struct definitions only contain data members, methods are added separately
const Pos = struct{ # struct{} is the syntax to create anonymous struct type
  x: int, 
  y: int
}

# Struct members can be given default values, the types will be inferred
const Other = struct{
  x = 12, # &int
  y = 32.1 # &float
}

let pos = .{x: 12, y: 13} # .{} is the syntax to create anonymous struct instances, type will be inferred
let pos = Pos{x: 12, y: 13}
# Functional updates, creates a copy of pos, with y changed to 11
let pos2 = .{...pos, y: 11}

let x = pos.x
let y = pos.y
let z = pos[:x]

std.testing.assert(x == 12 && y == 13 && z == 12)
```

- `Enum`  

Tagged unions, anonymous. If a tag is not given a type, it is given void. A integer type must be provided which is the type used for the tags behind the scenes
```elixir
let t = int
let e = string

const Result = enum{
  ok(t),
  err(e),
  other
}

let x = Result.ok(12)
let y = Result.err("Some error message")
let o = Result.other

# Enum can be pattern matched, to access inner values, errors if rhs is not the matching tag
let Result.ok(z) = x

std.testing.assert(z == 12)

# Enum can also be used for branching based on if the pattern matches or not
# The enum type can be inferred
if let .ok(z) = x {
  std.fmt.printf("{}", z)
}
```

## Methods and Receivers

Types can be given methods using receivers
```elixir
const Player = struct{
  pos: {f32, f32},
  health: int
}

# Methods for types are declared by specifying a reciever after the indentifier
# This can be used to add functionality to primitive types
const set_pos<exc p: &Player> = (pos: {f32, f32}) do: ...
const read_health<p: &Player> = (health: int) do: ...

# Receiver types can be normal types, pointer types, reference types, and compile time types
```

## Modules
In `Ruka`, modules are collections of bindings. Bindings can be let or const.
All modules are anonymous, named modules are made by storing modules in bindings
```elixir
const Constants = module{
  const PI = 3.14
}

let area = Constants.PI * (radius ** 2)
```
## Pattern Matching
```elixir
const Result = enum{
  ok(int),
  err(string)
}

let x = Result.ok(12);

match x {
  | Result.ok(val) do: std.fmt.println("{}", val),
  | .err(err) do: std.fmt.println(err),
  | _ {...} # Default case, not necissary here as all cases covered above
}
```

## Error Handling
```elixir
```

## File imports
When files are imported, they are stored as modules.
```elixir
const std = $import("std")
```

## More on functions
```elixir
# Functions can return multiple data types.
# Functions can return multiple pieces of data, 
#   but they must be assigned to multiple bindings when called.
# Return values can be given identifiers to declare bindings to use for returning, 
# allowing for naked returns
const div = (x, y: int): (quo, rem: int) {
  quo = x / y
  rem = x % y
  return
}

let quo, rem = div(12, 5)

# Returning a tuple or struct allows the return to be stored in a single binding
const div = (x, y: int): {int, int} {
  let quo = x / y
  let rem = x % y
  return {quo, rem}
}

let result = div(12, 5)
std.testing.assert(result[0] == 2)

const div = (x, y: int): struct{quo, rem: int} {
  let quo = x / y
  let rem = x % y
  return .{quo: quo, rem: rem} # if names match field tags, can ommit field name 
                                #   ie .{quo, rem}
}

let result = div(12, 5)
std.testing.assert(result.quo == 2)

# Functions can take variadic arguments using ...indent syntax.
# The arguments are packaged together into a tuple, which can then be indexed
const variadic = (...args) {
  let size = $len(args)
  for 0..<size |i| {
    std.fmt.printf("{} ", args[i])
  }
}

# Functions can be taken as parameters and returned from functions
const sort = (slice: []i32, pred: fn (i32, i32) -> bool) {
  ...
  if pred(slice[i], slice[j]) {
  ...
}

const arr = [41, 22, 31, 84, 75]
# The types of the anonymous function passed will be inferred
sort(arr[..], (lhs, rhs) do: lhs > rhs)

```

## Pipeline Operator
The `Pipeline` operator "|>" takes the result of the expression before it,
and inputs it into the first argument of the function after it
```elixir
const scan = (source: string): []tokens do: ...
const parse = (source: []tokens): Ast do: ...

let source = "some source code"

# Normally if you didnt want to save any of the intermediate steps you would write code like this.
let ast = parse(scan(source))
# Instead, this can be decomposed into steps, which follows the order of execution.
let ast = source
  |> scan()
  |> parse()

# This is similar to method chaining, an example of which is below, but using functions
#   which are separate from the data
let greeting = "!dlrow ,olleh"
  .reverse()
  .capitalize() # reverse and capitalize are methods of strings

```

## Behaviours
`Ruka` doesn't have inheritance, instead `Ruka` uses interfaces called `behaviours`.

Behaviours cannot specify data members, only methods
```elixir
# Behaviour definition
const Entity = behaviour{
  # Method types have restrictions on the receiver type, which goes after fn
  # Both of these methods require receivers to be exc& (a exclusive mode reference)
  update_pos: fn <exc&>({f32, f32}) -> void,
  update_health: fn <exc&>(int) -> void
}

const system = (exc entity: &Entity, ...) do: ...

# Behaviours are implemented implicitly
const Player = struct{
  # Members which are unique to each instance of the struct are declared like parameters
  pos: {f32, f32},
  health: int,
  ...,
}

# To implement the Entity Behaviour, it must have all methods defined with matching
#   identifiers, parameter types, and return types
const update_pos<exc p: &Player> = (pos: {f32, f32}) do: ...
const update_health<exc p: &Player> = (health: int) do: ...

let player = Player{} # If field values are not provided they will be set to the 
                       #   default values of that type, typically 0 or equivalent.
system(&player, ...)
```

## Compile Time
`Ruka` can run code at compile time instead of run time.

The return of compile time expressions can be stored in let, but they will no longer be usable in later compile time expressions
```elixir
# @ preceeding a identifier states that this parameter must be known at compile time
const Vector = (@t: typeid): typeid {
  return struct{
    x: t,
    y: t
  }
}

const t = int
# The function :Vector could be called at runtime:
let Pos = Vector(t) # This cannot be used in compile time expressions 
                     #   because it is executed at runtime
# Or compile time:
let Pos = @Vector(t) # This can no longer be used in later compile time expressions
const Pos = @Vector(t) # This can still be used in later compile time expressions

# Blocks can also be compile time
# Blocks can be specified with {...} or {...}
const screen_size = @ {
  return {1920, 1080}
}

## First Class Modules
Modules are first class in `Ruka`, so they can be passed into and out of functions
```elxir
# To create a generic ds with methods, you must return a struct with static bindings
const List = (@type: typeid): moduleid {
  return module{
    const t = struct{
      head: &Node,
      size: uint
    }

    const Node = struct{
      next: &Node,
      data: type
    }

    const insert<uni &t> = (value: type) {...}
  }
}

let intList = List(int).t{}
intList.insert(12)
```

## Operators
`Ruka` has many operators and symbols, some have different meaning depending on context:
<pre>
- Miscelaneous Operators
  - =   : Assignment 
  - []  : Index 
  - .   : Member Access 
  - /   : Namespace
  - ()  : Function Call 
  - &   : Reference/Address 
  - @   : Compile Time Expression 
  - *   : Dereference 
  - $   : Built in function
- Arithmetic Operators          - Wrapping - Saturating
  - +   : Addition                - +%      - +|
  - -   : Subtraction             - -%      - -|
  - *   : Multiplication          - *%      - *|
  - /   : Division                - /%      - /|
  - **  : Exponentiation          - ^%      - ^|
  - %   : Modulus or Remainder
  - ++  : Increment
  - --  : Decrement
  - Can be combined with the assignment operator, for example: += or ^|=
- Comparison Operators
  - >   : Greater than
  - >=  : Greater than or equal
  - <   : Less than
  - <=  : Less than or equal
  - ==  : Equal to
  - !=  : Not equal to
- Logical Operators
  - &&  : Logical And
  - ||  : Logical Or
  - !   : Logical Negation
- Bitwise Operators
  - &   : Bitwise AND
  - |   : Bitwise OR
  - ^   : Bitwise XOR
  - ~   : Bitwise Negation
- Type Symbols
  - type | type     : Union
  - !type           : Type or error
  - ?type           : Type or null
  - *type           : Pointer
  - &type           : Let Reference
  - []type          : Slice, which is a reference and a length
  - [size]type      : Array
  - [dyn]type       : Dynamic Array
  - %{key, value}   : Map
  - {type, ...}     : Tuple
  - list(type)      : List
  - range(type)     : Range, type must be integer types or byte
  - fn () -> ()     : Function
  - fn <>() -> ()   : Method
</pre>

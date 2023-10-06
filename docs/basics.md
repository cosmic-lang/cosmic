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
# A run-time constant/immutable binding, can be initialized after declaration.
let year: int
year = 2023
year = 2024 # Error, cannot reassign immutable binding
let year = 2024 # But the binding can be shadowed
```

- `var`  
```elixir
# Can be declared unitialized like let bindings.
var name: string
var age = 25
```

`Ruka` supports multiple assignment
```elixir
var x = 12
var y = 31
x, y = y, x # swaps bindings with no need for temporary bindings
```

Assignment in Ruka can also be done as an expression using ":=", which returns the rhs value
```elixir
var boolean: bool
# Assignment expression
while boolean := someFunc() { # Will loop until someFunc returns false 
  std/fmt.printf("{}", boolean)
}
```

Bindings of the same type can be grouped together.
``` elixir
# Var and let bindings still don't need to be initialized right away
var (
  x = 72,
  y
)

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
  - When  types are specified for bindings, the ":" is moved to the rhs

## Primitive Data Collections
`Ruka` has a few primitive data collections for you to use:
- `Array`
```elixir
# Arrays are static, their sizes cannot change and must be known at compile time
let arr = [1, 2, 3, 4, 5]
let num = arr[2]
std/testing.assert(num == 3)
```

- `Dynamic Array`
```elixir
# Can change size
let arr = [|1, 2, 3|]
let num = arr[1]
std/testing.assert(num == 2)
```

- `List`
```elixir
# Can change size
let list = {|1, 2, 3|}
let num = list[1]
std/testing.assert(num == 2)
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
std/testing.assert(carbon_mass == 15.999) # For floats == only compares the whole number
```

- `Tuple`  

Tuples can be indexed, or destructured using pattern matching. The $len() function can be used to assess the length of a tuple
```elixir
let pos = {10, 15}

std/testing.assert($len(pos) == 2)

let {x, y} = {pos[0], pos[1]}
let x, y = pos # The lhs braces are not required

std/testing.assert(x == 10 && y == 15)
```

## Blocks
Single-line blocks are written using do:
```elixir
do: let x = 83
```

Multi-line blocks are enclosed using braces: {}, or do end.  
```elixir
{
  let x = 83
}

do
  let x = 83
end

```

## Type specification basics

When declaring bindings, types are usually inferred based on later usage of the binding, 
but types can be specified if desired.

<pre>
  kind ident [: type] [= expression];
</pre>

If the binding is not initialized,
then a type specification must be added
```elixir
  let x = 83
  let x: int = 83

  var name: string
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

Anonymous function creation follows the form of:
<pre>
  (parameters [: type]) [: return type] body;
</pre>
the body is a block

Function definition follows the form of:
<pre>
  kind ident [(receiver: type)] [: type] = anonymous fn;
</pre>
the receiver will be covered later when methods are explained

A single-line body function
```elixir
const hello = (): string do: return "Hello, world!"
```

`{}` can be used for a multi line body. The final expression of a block is implicitly returned.
```elixir
const add = (x, y): int {
  x + y
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

const add_three = (x, y, z: int): int do: x + y + z
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

let pos = .{x: 12, y: 13} # .{} is the syntax to create anonymous struct instances, type will be inferred
let pos = Pos{x: 12, y: 13}
# Functional updates, creates a copy of pos, with y changed to 11
let pos2 = .{...pos, y: 11}

let x = pos.x
let y = pos.y
let z = pos[:x]

std/testing.assert(x == 12 && y == 13 && z == 12)
```

- `Enum`  

Tagged unions, if a tag is not given a type, it is given void. A integer type must be provided which is the type used for the tags behind the scenes
```elixir
let t = int
let e = string

const Result = enum{
  ok(t),
  err(e),
  other
}

var x = Result.ok(12)
var y = Result.err("Some error message")
var o = Result.other

# Enum can be pattern matched, to access inner values, errors if rhs is not the matching tag
var Result.ok(z) = x

std/testing.assert(z == 12)

# Enum can also be used for branching based on if the pattern matches or not
# The enum type can be inferred
if var .ok(z) = x {
  std/fmt.printf("{}", z)
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
const set_pos(self: &mut Player) = (pos: {f32, f32}) do: ...
const read_health(self: &Player) = (health: int) do: ...

# Receiver types can be normal types, pointer types, reference types, and compile time types
```

## Pattern Matching
```elixir
const Result = enum{
  ok(int),
  err(string)
}

var x = Result.ok(12);

match x {
| Result.ok(val) do: std/fmt.println("{}", val),
| .err(err) do: std/fmt.println(err),
| _ {...} # Default case, not necissary here as all cases covered above
}
```

## Error Handling
```elixir
```

## Memory Management
```elixir

```

## Modules
```elixir
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
  return # return could potentially be ommitted here
}

let quo, rem = div(12, 5)

# Returning a tuple or struct allows the return to be stored in a single binding
const div = (x, y: int): {int, int} {
  let quo = x / y
  let rem = x % y
  return {quo, rem}
}

let result = div(12, 5)
std/testing.assert(result[0] == 2)

const div = (x, y: int): struct{quo, rem: int} {
  let quo = x / y
  let rem = x % y
  return .{quo: quo, rem: rem} # if names match field tags, can ommit field name 
                                #   ie .{quo, rem}
}

let result = div(12, 5)
std/testing.assert(result.quo == 2)

# Functions can take variadic arguments using ...indent syntax.
# The arguments are packaged together into a tuple, which can then be indexed
const variadic = (...args) {
  let size = $len(args)
  for 0..<size |i| {
    std/fmt.printf("{} ", args[i])
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

var source = "some source code"

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
  update_pos: fn ({f32, f32}) -> (),
  update_health: fn (int) -> () 
}

const system = (entity: &Entity, ...) do: ...

# Behaviours are implemented implicitly
const Player = struct{
  # Members which are unique to each instance of the struct are declared like parameters
  pos: {f32, f32},
  health: int,
  ...,
}

# To implement the Entity Behaviour, it must have all methods defined with matching
#   identifiers, parameter types, and return types
const update_pos(self: &mut Player) = (pos: {f32, f32}) do: ...
const update_health(self: &mut Player) = (health: int) do: ...

var player = Player{} # If field values are not provided they will be set to the 
                       #   default values of that type, typically 0 or equivalent.
system(&player, ...)
```

## Compile Time
`Ruka` can run code at compile time instead of run time.

The return of compile time expressions can be stored in var or let, but they will no longer be usable in later compile time expressions
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
var Pos = @Vector(t) # This can no longer be used in later compile time expressions
const Pos = @Vector(t) # This can still be used in later compile time expressions

# Blocks can also be compile time
# Blocks can be specified with {...} or do...end
const screen_size = @ {
  return {1920, 1080}
}
```

## First Class Modules
```elixir
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
  - ^  : Bitwise XOR
  - ~   : Bitwise Negation
- Type Symbols
  - (type | type)   : Union
  - !type           : Type or error
  - ?type           : Type or void
  - *type           : Raw Pointer
  - []type          : Slice, which is a pointer and a length
  - &type           : Let Reference
  - &mut type       : Mutable Reference
  - [size]type      : Array
  - [dyn]type       : Dynamic Array
  - %{key, value}   : Map
  - {type, ...}     : Tuple
  - list(type)      : List
  - range(type)     : Range, type must be integer types or byte
  - fn () -> ()     : Function
</pre>

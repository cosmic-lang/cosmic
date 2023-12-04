# Language Syntax

## Maybe make language use semicolons, so assign expressions just don't end in semicolons

## Comments
```
# A single-line comment
```

## Bindings
Bindings in `Rex` follow the form of:  
<pre>
  kind tag [: type] [= expression]
</pre>
with the parts surrounded by [] being optional.  

## Binding Declaration and Initialization
There are two kinds of bindings:

- `const`  
```
# Constants must be assign a value when declared, and cannot be reassigned
const msg = "Hello, world!"
```

- `let`  
```
# A variable
let year = 2023
year = 2024
```

`Rex` supports multiple assignment
```
let x = 12
let y = 31
x, y = y, x # swaps bindings with no need for temporary bindings
```

Assignment in `Rex` can also be done as an expression using ":=", which returns the rhs value.
```
let boolean = false
# Assignment expression
while (boolean := someFunc()) { # Will loop until someFunc returns false 
    std/fmt.printf("{}", boolean)
}
```

Bindings of the same type can be grouped together.
``` 
# let bindings still don't need to be initialized right away
let (
    x = 72
    y
)
# If done on the same line, must be separated by commas
let (x, y)

```
## Type specification basics

When declaring bindings, types are usually inferred based on later usage of the binding, 
but types can be specified if desired.

<pre>
    kind tag [: type] [= expression];
</pre>

If the binding is not initialized,
then a type specification must be added.
```
let x = 83

let name: string
```

## Memory Management
In `Rex` memory is GC/stack allocated by default. Memory can be allocated manually using an allocator if desired. And GC can be disabled completely on a pre project basis.
- Manual management:
  - Using an allocator, you can manage memory manually, which will return a pointer to the memory which must be freed before the program ends
```
let name: int = 12 # GC/stack allocated

let names: *[5]string = std/mem/allocator.new([5]string) # Allocates an array and returns a pointer to it
defer std/mem/allocator.free(names) # Manual memory must be freed
```

## Basic Primitive Types
Here is a list of `Rex`'s primitive types:
- `isize`    
  - 12, architecture dependent size
- `i#`     
  - \# bit signed integer i.e. i16
- `usize` 
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
  - \\ Multi
    \\ line
    \\ string
- `regex`
  - \`foo|bar\`
- `bool` 
  - true, false
- `void` 
  - also ().
- `null`
- `typeid` 
  - i32, int, char, MyRecord. Types are values in `Rex`
- `moduleid`
- `error`
- `range` 
  - 0..10, 5...15
- `tag`   
  - :quick :skip
  - Polymorphic enums, i.e. don't need to be part of a type. 
  - Also used for identifiers, when used for identifiers the ":" can be omitted.
  - When used for map keys, the ":" is moved to the rhs
- `any`

## Primitive Data Collections
`Rex` has a few primitive data collections for you to use:
- `Array`
```
# Arrays are static, their sizes cannot change and must be known at compile time
let arr = [5]{1, 2, 3, 4, 5}
let num = arr[2]
std/testing.expect(num == 3)
```

- `Dynamic Array`
```
# Can change size
let arr = [dyn]{1, 2, 3}
let num = arr[1]
std/testing.expect(num == 2)
```

- `Tuple`  
Tuples can be indexed, or destructured using pattern matching. The $len() function can be used to assess the length of a tuple
```
let pos = {10, 15}

std/testing.expect($len(pos) == 2)

let {x, y} = {pos[0], pos[1]}
let x, y = pos # The lhs braces are not required

std/testing.expect(x == 10 and y == 15)
```
- `Named Tuple`
each {k, v} pair can be indexed, this is just syntactic sugar for creating tuples of two element tuples
```
let tagged_tuple = {name: "foo", age: 25, likes_ramen: true}
```

- `Map`
```
# Can change size
let atomic_mass: %{tag, f32} = %{
    beryllium: 9.1022,
    carbon: 15.999
}

let atomic_mass = %{
    "beryllium" => 9.1022,
    "carbon" => 15.999
}

let carbon_mass = atomic_mass[:carbon]
std/testing.expect(carbon_mass == 15.999) # For floats == only compares the whole number
```

## String interpolation
```
let fname = "foo"
let lname = "bar"

let name = "#{foo} #{bar}"
```

## String concatenation
```
let fname = "foo"
let lname = "bar"

let name = foo ++ " " ++ bar
```

## Blocks

Multi-line blocks are enclosed using braces: {}
```
{
    let x = 83
}
```

## Function Basics
All functions in `Rex` are anonymous closures, so function definition involves storing a function literal in a binding. Captured variables must be explicitly captured.

Anonymous function creation follows the form of:
<pre>
  ([mode] parameter [: type]) [: return type] => [|captures|] body;
</pre>
the body is a block

Function definition follows the form of:
<pre>
  kind tag [: type] = anonymous fn;
</pre>

A single-line body function
```
const hello = () => return "Hello, world!"
```
values must be returned explicitly

A multi line body.
```
const add = (x, y) => {
    return x + y
}
```
Parameters can be positional, or named. Named parameters must be declared with ~ preceding the tag. They are inferred to be optional types, but their types can be set to standard types. If used as optional types, must be after all positional parameters.
```
const add = (x, y) => {
    return x + y
}

add(1, 2) # x = 1, y = 2

const add = (x, ~y) => {
    return x + y
}

add(y: 1, 2) # y = 1, x = 2
add(1) # x = 1, y = null
```

## Universal Function Call Syntax
Functions can be called as methods, as long as the first parameter of the function
is the same type as the expression the "method" is being called on
```

```

## Error Handling
```
# Returns a result, which is a union (string or error)
const func1 = (): !string {
    if (...) {
        return error.someError
    }
}

# Returns a result, which is a union (void or error)
const func1 = (): !void {
    if (...) {
        return error.someError
    }
}

# Will throw exception if error
let s: string = func1() as string
# If error, returns error from current function
let s: string = func1().!

# Returns a optional, which is a union (int or null)
const func2 = (): ?int {

}

# Will throw exception if null
let i: int = func2() as int
# If null, returns error from current function
let i: int = func2().? 
# Null and false is treated as false, everything else is treated as true
# Can give a default value if return is null with or
let i: int = func2() or 12 
```

## Pattern Matching
```
const Result = enum {
    ok(int),
    err(string),
    other
}

let x = Result.ok(12)

match (x) {
    | Result.ok => |val| std/fmt.println("{}", val),
    | .err => |err| std/fmt.println(err),
    # Cases can be guarded using when followed by a condition
    # If the condition returns true, that case will execute
    | when is?(x) => |val| {}
}

let source = "int main() {}"

# The beginning of strings can be pattern matched,
# capturing the remaining portion of the string as a slice
match (source) {
    | "int", ... => |rest| {
        std/fmt.print("{}\n", rest) #" main() {}"
    }
}

let nums = [5]{1, 4, 2, 6, 8}

# Slices can be matched
match (nums[..]) {
    | [] => {
        # Matches an empty slice
    },
    | [] => |elem| {
        # Matches a slice with one element
    },
    | [..] => |elem, rest| {
        # Matches a slice with atleast two elements
    },
    | [..] => |_, rest| {
        # Captures can be ignored with "_"
    }
}

```

`Rex` also has a pattern matching operator `=~`, which returns rhs if pattern matches, otherwise returns null.
```
let input = "foo"
let reg = `foo|bar`

if (foo =~ reg) {

}

let tup = {52, 74, 412, 33, 87, 36}

if (pos ~= {_, 74, ...}) |rest| {

}

let nums = [5]{1, 2, 5, 3, 2}

if (nums[..] ~= []{1, 2, ...}) |rest| {

}
```


## Conditionals
```
if (condition) {

} else if (another_condition) {

} else if (variant =~ value) |inner| {

} else if (optional()) |not_null| {

} else if (result()) |not_error| {

} catch |error| {

} else {

}

unless (condition) {

}
```

## Loops
`Rex` has two looping constructs, range-based for loops, and while loops.
```
for (iterable, iterable2) |i, i2| {

}

while (condition) {

}

while (optional()) |value| {

}

while (result()) |value| {

} catch |error| {

}
```

## Function type specification
```
# Functions that take no parameters have empty "()" before the arrow.
# Void returns can be specified in two ways.
# The return type must always be specified in type specifications.
const foo: fn () -> ()
const bar: fn void -> void

# Types can be specified for multiple parameters at a time.
const add = (x, y: int): int => {
    return x + y
}

# Function types can be specified separately
$ fn (int, int, int) -> int
const add_three = (x, y, z) => return x + y + z
```

## Modes
Parameters can have constraints on them, called modes. Borrow types can only be mutated
in the scope they are defined in. Values passed to functions by borrow
cannot be mutated, unless they are passed in the unique or exclusive modes. This
may be able to be relaxed, so all values behind borrows can be modified
- Borrow types
  - `&` borrow mode, pass by reference
      - `loc` local mode, borrow cannot escape scope
      - `mov` unique mode, ownership of borrow is moved into function
      - `mut` exclusive mode, only one active borrow to value so safe to mutate
- All types
  - `@` or `ctime@` compile time mode
```
let x, y = 12, 11

const use = (mov& x: int) => {}

const add = (&x, y: int) => {
    use(&x)
    return x + y # Error x used after move
}

let name = "foo"

const rename = (mut& name: string) => {
    name = "bar"
}

rename(&name)
name # "bar"

```

## Creating new types
- `Record`  

All records are anonymous. Members can be accessed with the `.` operator. Members can also be accessed by indexing with a tag, provided the tag is known at compile time.
```
# Record definitions only contain data members, methods are added separately
const Pos = record { # record{} is the syntax to create anonymous record type
    x: int, 
    y: int
}

# Record members can be given default values, the types will be inferred
const Other = record {
    x = 12, # int
    y = 32.1 # float
}

let pos = .{x = 12, y = 13} # .{} is the syntax to create anonymous record instances, type will be inferred
let pos = Pos{x = 12, y = 13} # Can also specify name of record
# Functional updates, creates a copy of pos, with y changed to 11
let pos2 = .{...pos, y = 11}

let pos_x = pos.x
let pos_y = pos.y
let pos_z = pos[:x]
```

- `Variant`  

Tagged unions, anonymous. If a tag is not given a type, it is given void. Can specify tag integer type
```
const Result = enum {
    ok(int),
    err(string),
    other
}

let x = Result.ok(12)
let y = Result.err("Some error message")
let o = Result.other

# Variant can be pattern matched, to access inner values, errors if rhs is not the matching tag
let Result.ok(z) = x

std/testing.expect(z == 12)

# Variant can also be used for branching based on if the pattern matches or not
# The variant type can be inferred
if (.ok =~ x) |z| {
    std/fmt.printf("{}", z)
}
```

## Modules
In `Rex`, modules are collections of bindings. Bindings can be let or const.
All modules are anonymous, named modules are made by storing modules in bindings
```
const Constants = module {
    const PI = 3.14
}

let area = Constants.PI * (radius ** 2)
```
Modules can be extended using functional updates
```
const Constants = module {
    const PI = 3.14
}

const MoreConstants = module {
    ...Constants
    const TwoPi = Constants.PI * 2
    const Avogadros = 6.022e-23
}
```

## Methods and Receivers

Types can be given methods using receivers
```
const Player = record {
    pos: {f32, f32},
    health: int
}

# Methods for types are declared by specifying a reciever after the indentifier
# This can be used to add functionality to primitive types
const set_pos(mut& p: Player) = (pos: {f32, f32}) => self.pos = pos

# Receiver tag can be inferred to be self
const set_health(&Player) = (health: int) => self.health = health

# Can also be written using UFCS
const set_health = (&self: Player, health: int) => self.health = health

# And reciever can be inferred to be self
const set_health = (&Player, health: int) => self.health = health
```

## File imports
When files are imported, they are stored as modules.
```
const std = $import("std")
```

## Signals
Reactivity
```
# name: &string, update_name: signal
let (name, update_name) = $signal(string)
```

## Strings
Green threads
```
let sid = $spawn(() => {
    # Some code
})
defer sid.join()
```

## Channels
```
# name: &string, update_name: signal
let chan = $channel(string)

for (0..10) |i| {
    $spawn(() => |chan| {
        chan.send(i)
    })
}

let sum = 0
for (0..10) {
    sum += chan.receive()
}

for (chan.queue) |msg| {
    sum += msg
}
```

## More on functions
```
# Functions can return multiple data types.
# Functions can return multiple pieces of data, 
#   but they must be assigned to multiple bindings when called.
# Return values can be given tagifiers to declare bindings to use for returning, 
# allowing for naked returns
const div = (x, y: int): (quo, rem: int) => {
    quo = x / y
    rem = x % y
    return
}

let quo, rem = div(12, 5)

# Returning a tuple or record allows the return to be stored in a single binding
const div = (x, y: int): {int, int} => {
    let quo = x / y
    let rem = x % y
    return {quo, rem}
}

let result = div(12, 5)
std/testing.expect(result[0] == 2)

$ fn (int, int) -> record{quo, rem: int}
const div = (x, y) => {
    let quo = x / y
    let rem = x % y
    return .{quo = quo, rem = rem} # if names match field tags, can ommit field name 
                                   #   ie .{quo, rem}
}

let result = div(12, 5)
std/testing.expect(result.quo == 2)

# Any infers the function type at compile time where called, think templates
# If multiple args, they are treated as a tuple
# Must be the final argument
# ...tag can be used as shorthand for $any tuples
const variadic = (...args) => {
    let size = $len(args)

    for (0..size) |i| {
        std/fmt.println("{} ", args[i])
    }
}

const struct = (@tup: any) => {
    inline for ($typeOf(tup).members) |member| {

    }
}

@struct(.{...})

# Functions can be taken as parameters and returned from functions
const sort = (slice: []i32, pred: fn (i32, i32) -> bool) => {
    # code
    if pred(slice[i], slice[j]) {
    # code
}

const arr = [41, 22, 31, 84, 75]
# The types of the anonymous function passed will be inferred
sort(arr[..], (lhs, rhs) => lhs > rhs)

```

## Pipeline Operator
The `Pipeline` operator "|>" takes the result of the expression before it,
and inputs it into the first argument of the function after it
```
const scan = (source: string): []tokens => # code
const parse = (source: []tokens): Ast => # code

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

## Traits
`Rex` doesn't have inheritance, instead `Rex` uses interfaces called `traits`.

Traits cannot specify data members, only methods
```
# Trait definition
const Entity = trait {
    # Method types have restrictions on the receiver type, which goes after fn
    # Both of these methods require receivers to be &'e' (a exclusive mode borrow)
    update_pos: fn (mut&)({f32, f32}) -> (),
    update_health: fn (mut&)(int) -> ()
}

const system = (mut& entity: Entity) => # code

# Traits are implemented implicitly
const Player = record {
    # Members which are unique to each instance of the record are declared like parameters
    pos: {f32, f32},
    health: int,
    ...,
}

# To implement the Entity Behaviour, it must have all methods defined with matching
#   tagifiers, parameter types, and return types
const update_pos = (mut& Player, pos: {f32, f32}) => # code
const update_health = (mut& Player, health: int) => # code

let player = Player{} # If field values are not provided they will be set to the 
                       #   default values of that type, typically 0 or equivalent.
system(&player)
```

## `ctime` Expressions
Metaprogramming in `Rex` is done using ctime expressions, which is just `Rex` code executed at compile time

The return of compile time expressions is a reference to a static variable
```
# `@` or `ctime@` preceeding a tagifier states that this parameter must be known at compile time
const Vector = (ctime@t: typeid): typeid => {
    return record{
        x: t,
        y: t
    }
}

const t = int
# The function :Vector could be called at runtime:
let Pos = Vector(t) # This cannot be used in meta expressions 
                     #   because it is executed at runtime
# Or compile time (@ is used to run an expression at ctime):
let Pos = @Vector(t) # This can be used in later compile time expressions as long as it is not assigned to again
const Pos = @Vector(t) # This can be used in later compile time expressions

# Blocks can also be run at compile time
const screen_size = @{
    return {1920, 1080}
}
```
## First Class Modules
Modules are first class in `Rex`, so they can be passed into and out of functions
```
# To create a generic ds with methods, you must return a record with static bindings
const List = (ctime@type: typeid): moduleid => {
    return module {
        const Node = record {
            next: $this(),
            data: type
        }   
    
        pub const t = record {
            head: Node,
            size: uint
        }

        const insert = (mut& t, value: type) => {...}
    }
};

let intList = List(int).t{}
intList.insert(12)
```

## Operators
`Rex` has many operators and symbols, some have different meaning depending on context:
```
- Miscelaneous Operators
  - /   : Namespace Resolution
  - =   : Assignment 
  - :=  : Assignment Expression
  - =~  : Pattern Match
  - !~  : Pattern Not Match
  - []  : Index 
  - .   : Member Access 
  - ()  : Function Call 
  - &   : Borrow 
  - @   : Ctime Mode
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
  - and : Logical And
  - or  : Logical Or
  - not : Logical Negation
- Bitwise Operators
  - &   : Bitwise AND
  - |   : Bitwise OR
  - ^   : Bitwise XOR
  - !   : Bitwise Negation
  - <<  : Bitshift Left
  - >>  : Bitshift Right
- Type Symbols
  - any             : Ctime Inferred type
  - (type or type)  : Union
  - !type           : type or error
  - ?type           : type or null
  - *type           : Pointer
  - []type          : Slice, which is a pointer and a length
  - [size]type      : Array
  - [dyn]type       : Dynamic Array
  - {type, ...}     : Tuple
  - %{key, value}   : Map
  - ..(type)        : Exclusive Range, type must be integer types or byte
  - ...(type)       : Inclusive Range, type must be integer types or byte
  - fn () -> ()     : Function
  - fn ()() -> ()   : Trait Method
```

## Example: Linked List
```
const List = (ctime@type: typeid): moduleid => {
    return module {
        let max_size = 100

        const node = record {
            next: ?$this(),
            data: type
        }

        pub const t = record {
            head: ?node,
            size: uint
        }

        const insert(mut& t) = (value: type) => |max_size| {
            if (self.size == 0) {
                self.head = node {
                    next: null,
                    data: value
                }
                self.size++ 
            } else if (self.size <= max_size) {
                let tmp = self.head

                self.head = node {
                    next: tmp,
                    data: value
                }
                self.size++ 
            }
        }

        const set_max = (size: usize) => |max_size| {
            max_size.* = size
        }
    }
}

let names = List(string).t{}

names.insert("foo")
names.insert("bar")
names.insert("foobar")
```

## Circuits
`Rex` has an extension called `Silver`, which integrates HDL into the language for simple FPGA development.

Refer to `Silver` for details
```
# Hardware circuit instantiation must be done at compile time
# Ports will connect to mmio
# The returned structure contains functions to interact w/ hardware through the mmio

# This creates a circuit type
const AndGate = circuit { 
    port (
        x(in: u1)
        y(in: u1)
        z(out: u1)
    )
  
    arch (
        z = x & y
    )
}

let and = @AndGate{} # This creates an instance of AndGate, 
                     # which must be done at compile time

and.put(x: 1, y: 1)

let result = and.get(:z) # Output ports are setup with signals,
                         # so reading from a output port blocks 
                         # execution until the signal is high
```

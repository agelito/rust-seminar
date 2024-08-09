---
title: 3. Rust Syntax
---

# Rust Syntax
Rust is a systems language but sometimes feels like writing high level code. The syntax varies from very simple to extremely complex. Extremely optimized Rust code can be complicated to read but most of the time it's simple.

## Functions
Functions in Rust is declared using the `fn` keyword followed by function name and parameter list, optionally a return value type can also be defined. The function body is surrounded by `{` `}` braces.

```rust
fn hello() {
    println!("Hello!");
}

fn sum(a: usize, b: usize) -> usize {
    a + b
}
```

It's also possible to declare the visibility of functions by adding visibility keywords in front of fn.

```rust
// public function
pub fn goodbye() {
    println!("Goodbye!");
}

// public function in crate scope
pub(crate) fn foo() -> &'static str {
    "bar"
}
```

Reading these examples you may have noticed there's no `return` statement, even though some of the functions return something. In Rust the `return` statement is optional and if the last expression inside a function is not ended with a `;` it will be implicitly returned.

```rust
fn a_number() -> usize {
    69
}

fn another_number() -> usize {
    return 420;
}
```

Both of the previous examples returns a number with the type `usize`. Note the missing `;` in the `a_number` function.

## Variables
Variables in Rust is immutable by default, this means once they've been assigned a value they can't change. 

```rust
let x = 42;
```

The type of `x` in the previous example is implicitly determined by the compiler. It's also possible to explicitly declare what type a variable should have.

```rust
let x: usize = 42;
```

Trying to reassign the value of `x` in the previous example would result in a compilation error. To get around this the variables can be marked _mutable_ using the `mut` keyword.

```rust
let mut x = 42;
x = 69;
```

## Program Control Flow
Rust has if-statements and loops to control the program flow.

```rust
if x == 42 {
    println!("x is 42");
} else {
    println!("x is not 42");
}
```

```rust
// Iterate over range of numbers
for x in 0..100 {
    println!("{}", x);
}

// Endless loop
loop {
    break; // the `break` keyword will exit any loop
}

// While loop
let running = true;
while running {
    running = false;    
}

```

## Pattern Matching
A very powerful concept in Rust is pattern matching, the Rust compiler ensures every pattern is matched.

```rust
let optional = Some(42);

match optional {
    Some(42) => { println!("matched 42"); },
    Some(_) => { println!("matched another number"); },
    None => { println!("there is no number") },
}
```

Patterns or values can be skipped by using the `_` character.

```rust
let n = 69;

match n {
    69 => { println!("niiceee"); }
    _ => { println!("meh"); }
}
```

## Others
These examples just scratches the surface of some of the more common syntax in Rust. The scope of this seminar doesn't include every single example. When writing or reading code and coming across some syntax that looks weird, please come and talk to me and I will be happy to explain or point to resources explaining it.


# Move Semantics in Rust

## Overview
Understanding Rust's ownership system, moves, borrows, and how memory management works without a garbage collector.

## Key Concepts
- Ownership rules
- Move semantics
- Borrowing (references)
- Mutable vs immutable references
- Lifetimes basics
- Copy vs Move traits

## Learning Objectives
- Understand when values are moved vs copied
- Use references to avoid unnecessary moves
- Handle ownership transfer correctly
- Understand the borrow checker's role
- Write memory-safe code without garbage collection

## Common Patterns
```rust
// Move semantics with String
let s1 = String::from("hello");
let s2 = s1;                    // s1 is moved to s2, s1 is no longer valid
// println!("{}", s1);          // This would cause a compile error

// Borrowing instead of moving
let s3 = String::from("world");
let len = calculate_length(&s3); // Borrow s3, don't move it
println!("The length of '{}' is {}.", s3, len); // s3 is still valid

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but nothing happens because it's just a reference

// Mutable references
let mut s = String::from("hello");
change(&mut s);
println!("{}", s); // Prints "hello, world"

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Copy types (integers, booleans, etc.)
let x = 5;
let y = x;                      // Copy, not move
println!("x = {}, y = {}", x, y); // Both are still valid

// Cloning to avoid moves
let s1 = String::from("hello");
let s2 = s1.clone();            // Explicit copy
println!("s1 = {}, s2 = {}", s1, s2); // Both are valid
```

## Ownership Rules
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

## Borrowing Rules
1. You can have either one mutable reference or any number of immutable references
2. References must always be valid (no dangling references)

## Best Practices
- Pass large data structures by reference when you don't need ownership
- Use mutable references when you need to modify data
- Clone only when necessary (it's expensive for large data)
- Let the compiler guide you with helpful error messages
- Understand when types implement Copy vs requiring moves

## Common Mistakes
- Trying to use a value after it's been moved
- Having multiple mutable references to the same data
- Mixing mutable and immutable references incorrectly
- Creating dangling references

## Advanced Patterns
```rust
// Returning references with lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Moving out of collections
let mut v = vec![String::from("hello"), String::from("world")];
let first = v.remove(0);        // Move out of vector
// let first = &v[0];           // Borrow instead

// Partial moves
struct Person {
    name: String,
    age: u32,
}

let person = Person {
    name: String::from("Alice"),
    age: 30,
};

let name = person.name;         // Move name out
// let age = person.age;        // Can still access age (Copy type)
// println!("{}", person.name); // Error: name was moved
```

## Additional Resources
- [The Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - Move Semantics](https://doc.rust-lang.org/rust-by-example/scope/move.html)
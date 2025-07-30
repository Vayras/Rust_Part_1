# Conditional Statements in Rust

## Overview
Learn about if expressions, conditional logic, and pattern matching basics in Rust.

## Key Concepts
- `if` expressions
- `else if` and `else` clauses
- Using `if` in `let` statements
- Boolean conditions
- Comparison operators

## Learning Objectives
- Write conditional statements using `if`
- Use `if` expressions to assign values
- Combine multiple conditions
- Understand Rust's expression-based conditionals

## Common Patterns
```rust
// Basic if statement
let number = 6;
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else {
    println!("number is not divisible by 4 or 3");
}

// Using if in a let statement
let condition = true;
let number = if condition { 5 } else { 6 };

// Complex conditions
let x = 10;
let y = 20;
if x < y && y > 15 {
    println!("Both conditions are true");
}

// Nested if expressions
let weather = "sunny";
let temperature = 25;
let message = if weather == "sunny" {
    if temperature > 20 {
        "Perfect day for a picnic!"
    } else {
        "Sunny but a bit cold"
    }
} else {
    "Maybe stay inside"
};
```

## Best Practices
- Use parentheses sparingly (Rust doesn't require them around conditions)
- Keep conditions readable and simple
- Use early returns to reduce nesting
- Consider using `match` for complex conditional logic
- Ensure both branches of an `if` expression return the same type

## Common Mistakes
- Using assignment (`=`) instead of comparison (`==`)
- Forgetting that `if` is an expression and can return values
- Mixing types in different branches of an `if` expression

## Additional Resources
- [The Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust by Example - if/else](https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html)
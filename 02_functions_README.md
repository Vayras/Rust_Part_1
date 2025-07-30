# Functions in Rust

## Overview
Master function definition, parameters, return values, and Rust's expression-based nature.

## Key Concepts
- Function definition with `fn`
- Parameters and arguments
- Return values and return types
- Statements vs expressions
- Early returns

## Learning Objectives
- Define and call functions
- Pass parameters to functions
- Return values from functions
- Understand the difference between statements and expressions
- Use early returns effectively

## Common Patterns
```rust
// Basic function
fn greet() {
    println!("Hello!");
}

// Function with parameters
fn add(x: i32, y: i32) -> i32 {
    x + y  // Expression (no semicolon)
}

// Function with early return
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        return None;  // Early return
    }
    Some(x / y)
}

// Function with multiple return points
fn categorize_number(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    if n == 0 {
        return "zero";
    }
    "positive"
}
```

## Best Practices
- Use descriptive function names
- Keep functions small and focused
- Use type annotations for parameters and return types
- Prefer expressions over statements for return values
- Use early returns to reduce nesting

## Common Mistakes
- Adding semicolons to the last expression (making it a statement)
- Forgetting to specify return types
- Not handling all possible return paths

## Additional Resources
- [The Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust by Example - Functions](https://doc.rust-lang.org/rust-by-example/fn.html)
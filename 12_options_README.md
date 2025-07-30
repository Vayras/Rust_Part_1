# Option Type in Rust

## Overview
Master Rust's approach to handling nullable values safely with the Option enum, eliminating null pointer errors.

## Key Concepts
- `Option<T>` enum with `Some(T)` and `None` variants
- Pattern matching with `match`
- Convenience methods: `unwrap()`, `expect()`, `unwrap_or()`
- Chaining operations with `map()`, `and_then()`, `filter()`
- Converting between `Option` and `Result`

## Learning Objectives
- Use Option to represent optional values
- Handle Option values safely without panicking
- Chain operations on Option values
- Understand when to use Option vs other approaches
- Convert between different Option representations

## Common Patterns
```rust
// Creating Option values
let some_number = Some(5);
let some_string = Some(String::from("hello"));
let absent_number: Option<i32> = None;

// Pattern matching
fn describe_option(x: Option<i32>) -> String {
    match x {
        Some(value) => format!("Got a value: {}", value),
        None => String::from("Got nothing"),
    }
}

// Using if let for single variant matching
let maybe_value = Some(42);
if let Some(value) = maybe_value {
    println!("Found value: {}", value);
}

// Convenience methods
let x = Some(10);
let y: Option<i32> = None;

// Unwrapping (use carefully - can panic!)
let value = x.unwrap();                    // Panics if None
let value = x.expect("x should have a value"); // Custom panic message

// Safe unwrapping
let value = x.unwrap_or(0);                // Use 0 if None
let value = x.unwrap_or_else(|| expensive_computation());
let value = y.unwrap_or_default();         // Use type's default value

// Checking if Option has a value
if x.is_some() {
    println!("x has a value");
}
if y.is_none() {
    println!("y is empty");
}
```

## Functional Programming with Option
```rust
let numbers = vec![Some(1), None, Some(3), Some(4), None];

// Map over Option values
let doubled: Vec<Option<i32>> = numbers
    .iter()
    .map(|opt| opt.map(|x| x * 2))
    .collect();

// Filter out None values and unwrap
let valid_numbers: Vec<i32> = numbers
    .into_iter()
    .filter_map(|opt| opt)  // or .flatten()
    .collect();

// Chaining operations
let result = Some(10)
    .map(|x| x * 2)                        // Some(20)
    .filter(|&x| x > 15)                   // Some(20)
    .map(|x| x.to_string());               // Some("20")

// and_then for chaining functions that return Option
fn divide(x: f64, y: f64) -> Option<f64> {
    if y != 0.0 {
        Some(x / y)
    } else {
        None
    }
}

let result = Some(10.0)
    .and_then(|x| divide(x, 2.0))         // Some(5.0)
    .and_then(|x| divide(x, 0.0));        // None
```

## Working with Collections
```rust
// Finding elements
let numbers = vec![1, 2, 3, 4, 5];
let found = numbers.iter().find(|&&x| x > 3);    // Some(&4)
let position = numbers.iter().position(|&x| x == 3); // Some(2)

// Getting elements by index
let first = numbers.get(0);                // Some(&1)
let out_of_bounds = numbers.get(10);       // None

// Working with HashMaps
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert("Blue", 10);

let score = scores.get("Blue");            // Some(&10)
let missing = scores.get("Red");           // None

// Updating values
let entry = scores.entry("Yellow").or_insert(50);
*entry += 10;
```

## Best Practices
- Use `Option` instead of nullable pointers or special values
- Prefer pattern matching or functional methods over `unwrap()`
- Use `expect()` with descriptive messages when you're confident the value exists
- Chain operations with `map()`, `and_then()`, etc. for cleaner code
- Use `?` operator when working with functions that return `Option`

## Common Mistakes
- Using `unwrap()` without being certain the value exists
- Not handling the `None` case in pattern matches
- Overusing `is_some()` and `is_none()` instead of pattern matching
- Converting Option to Result unnecessarily

## Converting Between Option and Result
```rust
// Option to Result
let maybe_value = Some(10);
let result: Result<i32, &str> = maybe_value.ok_or("No value found");

// Result to Option
let result: Result<i32, &str> = Ok(42);
let option = result.ok();                  // Some(42)

let error_result: Result<i32, &str> = Err("Something went wrong");
let option = error_result.ok();            // None
```

## Additional Resources
- [The Rust Book - Option](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [Rust by Example - Option](https://doc.rust-lang.org/rust-by-example/std/option.html)
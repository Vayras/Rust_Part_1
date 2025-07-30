# Error Handling in Rust

## Overview
Learn Rust's approach to error handling using the Result type, custom errors, and the ? operator for robust error propagation.

## Key Concepts
- `Result<T, E>` type with `Ok(T)` and `Err(E)` variants
- The `?` operator for error propagation
- Custom error types
- `panic!` macro for unrecoverable errors
- Error trait and error chaining
- Converting between error types

## Learning Objectives
- Handle recoverable errors with Result
- Use the ? operator for clean error propagation
- Create and use custom error types
- Understand when to panic vs return errors
- Chain and convert different error types

## Common Patterns
```rust
use std::fs::File;
use std::io::{self, Read};

// Basic Result handling
fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // ? propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Pattern matching on Result
fn handle_file_operation() {
    match read_file_contents("hello.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }
}

// Using convenience methods
fn process_file(filename: &str) {
    let contents = read_file_contents(filename)
        .unwrap_or_else(|_| String::from("Default content"));
    
    // Or with expect for better error messages
    let contents = read_file_contents(filename)
        .expect("Failed to read the file");
}

// Chaining Results
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
        .map(|n| n * 2)
}

// and_then for chaining fallible operations
fn divide_and_parse(x: &str, y: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let x_num: f64 = x.parse()?;
    let y_num: f64 = y.parse()?;
    
    if y_num == 0.0 {
        return Err("Division by zero".into());
    }
    
    Ok(x_num / y_num)
}
```

## Custom Error Types
```rust
use std::fmt;

// Simple custom error
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
        }
    }
}

impl std::error::Error for MathError {}

// Using custom errors
fn safe_divide(x: f64, y: f64) -> Result<f64, MathError> {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// More complex custom error with context
#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
}

impl ValidationError {
    fn new(field: &str, message: &str) -> Self {
        ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation error in field '{}': {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}
```

## Error Propagation and Conversion
```rust
// Using ? with different error types
fn process_data(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(input)?;  // IO error
    let number: i32 = file_contents.trim().parse()?;      // Parse error
    let result = safe_divide(number as f64, 2.0)?;        // Custom error
    Ok(result as i32)
}

// Manual error conversion
impl From<std::io::Error> for MathError {
    fn from(_error: std::io::Error) -> Self {
        MathError::DivisionByZero  // Simplified conversion
    }
}

// Using Result combinators
fn complex_operation(x: i32, y: i32) -> Result<String, MathError> {
    safe_divide(x as f64, y as f64)
        .and_then(|result| safe_sqrt(result))
        .map(|final_result| format!("Result: {:.2}", final_result))
}
```

## Best Practices
- Use `Result` for recoverable errors, `panic!` for programming errors
- Create meaningful custom error types for your domain
- Use the `?` operator for clean error propagation
- Provide context in error messages
- Consider using error handling libraries like `anyhow` or `thiserror`
- Handle errors at the appropriate level in your application

## Common Mistakes
- Using `unwrap()` in production code without good reason
- Creating errors that don't implement the Error trait
- Not providing enough context in error messages
- Mixing different error handling strategies inconsistently
- Using `panic!` for recoverable errors

## Advanced Error Handling
```rust
// Using the `anyhow` crate (add to Cargo.toml)
use anyhow::{Result, Context};

fn read_and_process_file(path: &str) -> Result<String> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    let processed = contents
        .lines()
        .map(|line| line.to_uppercase())
        .collect::<Vec<_>>()
        .join("\n");
    
    Ok(processed)
}

// Error chaining
fn nested_operation() -> Result<()> {
    outer_operation()
        .context("Failed in nested operation")?;
    Ok(())
}

fn outer_operation() -> Result<()> {
    inner_operation()
        .context("Failed in outer operation")?;
    Ok(())
}

fn inner_operation() -> Result<()> {
    Err(anyhow::anyhow!("Something went wrong"))
}
```

## Additional Resources
- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [anyhow crate](https://docs.rs/anyhow/)
- [thiserror crate](https://docs.rs/thiserror/)
# Primitive Types in Rust

## Overview
Explore Rust's built-in primitive types including integers, floats, booleans, characters, and compound types.

## Key Concepts
- Integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`)
- Floating-point types (`f32`, `f64`)
- Boolean type (`bool`)
- Character type (`char`)
- Tuple types
- Array types
- Slices

## Learning Objectives
- Choose appropriate numeric types for different use cases
- Work with tuples and arrays
- Understand the difference between arrays and slices
- Use type inference and explicit type annotations

## Common Patterns
```rust
// Integer types
let x: i32 = 42;           // 32-bit signed integer
let y: u64 = 1000;         // 64-bit unsigned integer
let z = 100_000;           // Type inferred as i32, with underscores for readability

// Floating-point types
let pi: f64 = 3.14159;     // 64-bit float (default)
let e: f32 = 2.718;        // 32-bit float

// Boolean
let is_active: bool = true;
let is_finished = false;    // Type inferred

// Character (Unicode scalar values)
let letter: char = 'A';
let emoji: char = '😀';

// Tuples
let point: (i32, i32) = (10, 20);
let (x, y) = point;         // Destructuring
let mixed: (i32, f64, char) = (42, 3.14, 'A');

// Arrays (fixed size)
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];        // Array of 10 zeros
let first = numbers[0];     // Indexing

// Slices (dynamic size)
let slice: &[i32] = &numbers[1..4];  // Elements 1, 2, 3
let all: &[i32] = &numbers[..];      // All elements
```

## Best Practices
- Use `i32` for most integer values unless you need a specific size
- Use `f64` for floating-point values (it's the default)
- Use `usize` for indexing and sizes
- Use descriptive variable names for tuples or consider structs for complex data
- Prefer arrays when size is known at compile time, slices for dynamic access

## Common Mistakes
- Integer overflow in debug mode (panics) vs release mode (wraps)
- Mixing signed and unsigned integers without explicit conversion
- Accessing array elements out of bounds
- Confusing arrays (stack-allocated, fixed size) with vectors (heap-allocated, dynamic size)

## Type Conversions
```rust
let x: i32 = 10;
let y: i64 = x as i64;      // Explicit casting
let z: f64 = x as f64;      // Integer to float

// Safe conversions
let a: u32 = 100;
let b: i32 = a.try_into().unwrap();  // May panic if conversion fails
```

## Additional Resources
- [The Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust by Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
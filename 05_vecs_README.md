# Vectors in Rust

## Overview
Learn about Rust's growable arrays - vectors - and how to work with dynamic collections of data.

## Key Concepts
- Creating vectors with `Vec::new()` and `vec![]` macro
- Adding elements with `push()`
- Accessing elements safely
- Iterating over vectors
- Vector capacity and length
- Removing elements

## Learning Objectives
- Create and initialize vectors
- Add, remove, and access vector elements
- Iterate over vectors in different ways
- Understand ownership with vectors
- Handle potential panics when accessing elements

## Common Patterns
```rust
// Creating vectors
let mut v1: Vec<i32> = Vec::new();           // Empty vector
let mut v2 = vec![1, 2, 3, 4, 5];           // Vector with initial values
let v3 = vec![0; 10];                       // Vector with 10 zeros

// Adding elements
v1.push(10);
v1.push(20);
v1.extend([30, 40, 50]);                    // Add multiple elements

// Accessing elements
let first = &v2[0];                         // Direct indexing (may panic)
let second = v2.get(1);                     // Safe access, returns Option<&T>

match v2.get(10) {
    Some(value) => println!("Value: {}", value),
    None => println!("Index out of bounds"),
}

// Iterating
for item in &v2 {                           // Immutable reference
    println!("{}", item);
}

for item in &mut v1 {                       // Mutable reference
    *item += 10;
}

for (index, value) in v2.iter().enumerate() {
    println!("Index {}: {}", index, value);
}

// Removing elements
v1.pop();                                   // Remove last element
v1.remove(0);                               // Remove element at index
v1.clear();                                 // Remove all elements

// Vector properties
println!("Length: {}", v2.len());
println!("Capacity: {}", v2.capacity());
println!("Is empty: {}", v2.is_empty());
```

## Best Practices
- Use `vec![]` macro for vectors with initial values
- Use `Vec::with_capacity()` if you know the approximate size
- Prefer `get()` over direct indexing for safe access
- Use iterators instead of manual indexing when possible
- Consider using `Vec::reserve()` to avoid reallocations

## Common Mistakes
- Direct indexing without bounds checking (can cause panics)
- Trying to modify a vector while iterating over it
- Not understanding when vectors move vs when they're borrowed
- Forgetting that `pop()` returns an `Option<T>`

## Advanced Patterns
```rust
// Collecting from iterators
let numbers: Vec<i32> = (0..10).collect();
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// Filtering
let evens: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

// Finding elements
let found = numbers.iter().find(|&&x| x > 5);
let position = numbers.iter().position(|&x| x == 7);

// Sorting
let mut names = vec!["Alice", "Bob", "Charlie"];
names.sort();                               // Sort in place
names.sort_by(|a, b| b.cmp(a));            // Custom sorting (reverse)

// Deduplication
let mut values = vec![1, 2, 2, 3, 3, 3, 4];
values.dedup();                             // Remove consecutive duplicates
```

## Memory Considerations
- Vectors are heap-allocated
- They automatically grow when needed
- Growing involves reallocation and copying
- Use `capacity()` to check allocated space vs `len()` for actual elements

## Additional Resources
- [The Rust Book - Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Rust by Example - Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)
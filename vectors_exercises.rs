// Vectors - Custom Exercises
// Complete each exercise by replacing the TODO comments with working code

// Exercise 1: Creating and Basic Operations
fn exercise1() {
    println!("=== Exercise 1: Vector Basics ===");
    
    // TODO: Create an empty vector of integers
    
    // TODO: Create a vector with initial values [1, 2, 3, 4, 5]
    
    // TODO: Create a vector of 10 zeros using vec![value; count] syntax
    
    // TODO: Print all three vectors
    
}

// Exercise 2: Adding and Removing Elements
fn exercise2() {
    println!("=== Exercise 2: Modifying Vectors ===");
    
    // TODO: Create a mutable empty vector of strings
    
    // TODO: Add "apple", "banana", "cherry" using push()
    
    // TODO: Add multiple fruits at once using extend()
    // extend with ["date", "elderberry"]
    
    // TODO: Remove the last element using pop() and print what was removed
    
    // TODO: Remove the first element using remove() and print what was removed
    
    // TODO: Print the final vector
    
}

// Exercise 3: Accessing Elements Safely
fn exercise3() {
    println!("=== Exercise 3: Safe Element Access ===");
    
    let numbers = vec![10, 20, 30, 40, 50];
    
    // TODO: Access the first element safely using get() and handle the Option
    
    // TODO: Try to access element at index 10 safely and handle the None case
    
    // TODO: Access the third element using direct indexing (numbers[2])
    
    // TODO: Demonstrate what happens when you try to access out of bounds
    // (Comment this out after testing to avoid panic)
    // let out_of_bounds = numbers[10];
    
}

// Exercise 4: Iterating Over Vectors
fn exercise4() {
    println!("=== Exercise 4: Vector Iteration ===");
    
    let mut scores = vec![85, 92, 78, 96, 88];
    
    // TODO: Iterate over the vector with immutable references and print each score
    
    // TODO: Iterate with indices using enumerate() and print "Score 1: 85" format
    
    // TODO: Iterate with mutable references and add 5 points to each score
    
    // TODO: Print the modified scores
    
}

// Exercise 5: Vector Operations and Methods
fn exercise5() {
    println!("=== Exercise 5: Vector Methods ===");
    
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    
    // TODO: Print the length of the vector
    
    // TODO: Check if the vector is empty
    
    // TODO: Find the first occurrence of the number 5
    
    // TODO: Find the position of the number 9
    
    // TODO: Check if the vector contains the number 7
    
    // TODO: Create a new vector with only even numbers using filter and collect
    
    // TODO: Create a new vector with all numbers doubled using map and collect
    
}

// Exercise 6: Vector Sorting and Deduplication
fn exercise6() {
    println!("=== Exercise 6: Sorting and Dedup ===");
    
    let mut names = vec!["Charlie", "Alice", "Bob", "Alice", "David", "Bob"];
    
    // TODO: Sort the vector in alphabetical order
    
    // TODO: Print the sorted vector
    
    // TODO: Remove consecutive duplicates (note: sort first for effective dedup)
    
    // TODO: Print the deduplicated vector
    
    // TODO: Sort in reverse order using sort_by
    
    // TODO: Print the reverse sorted vector
    
}

// Exercise 7: Working with Different Types
fn exercise7() {
    println!("=== Exercise 7: Different Vector Types ===");
    
    // TODO: Create a vector of tuples representing (name, age)
    
    // TODO: Add at least 3 people to the vector
    
    // TODO: Find and print the person with the highest age
    
    // TODO: Create a vector of the names only (extract first element of each tuple)
    
    // TODO: Create a vector of boolean values and count how many are true
    
}

// Exercise 8: Advanced Vector Operations
fn exercise8() {
    println!("=== Exercise 8: Advanced Operations ===");
    
    let numbers1 = vec![1, 2, 3];
    let numbers2 = vec![4, 5, 6];
    
    // TODO: Concatenate the two vectors into a new vector
    // Hint: Use extend() or chain() and collect()
    
    // TODO: Create a vector of vectors (2D vector) and add the above vectors to it
    
    // TODO: Flatten the 2D vector back into a 1D vector
    
    // TODO: Partition a vector of numbers into evens and odds
    let mixed = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Hint: Use partition() method
    
}

fn main() {
    exercise1();
    exercise2();
    exercise3();
    exercise4();
    exercise5();
    exercise6();
    exercise7();
    exercise8();
    
    println!("\n🎉 All vector exercises completed!");
}

// Solutions (uncomment to check your work):
/*
fn exercise1_solution() {
    println!("=== Exercise 1: Vector Basics ===");
    
    let empty_vec: Vec<i32> = Vec::new();
    let numbers = vec![1, 2, 3, 4, 5];
    let zeros = vec![0; 10];
    
    println!("Empty vector: {:?}", empty_vec);
    println!("Numbers: {:?}", numbers);
    println!("Zeros: {:?}", zeros);
}

fn exercise2_solution() {
    println!("=== Exercise 2: Modifying Vectors ===");
    
    let mut fruits: Vec<String> = Vec::new();
    
    fruits.push(String::from("apple"));
    fruits.push(String::from("banana"));
    fruits.push(String::from("cherry"));
    
    fruits.extend(["date", "elderberry"].iter().map(|s| s.to_string()));
    
    if let Some(removed) = fruits.pop() {
        println!("Removed last: {}", removed);
    }
    
    let first = fruits.remove(0);
    println!("Removed first: {}", first);
    
    println!("Final fruits: {:?}", fruits);
}

fn exercise3_solution() {
    println!("=== Exercise 3: Safe Element Access ===");
    
    let numbers = vec![10, 20, 30, 40, 50];
    
    match numbers.get(0) {
        Some(value) => println!("First element: {}", value),
        None => println!("No first element"),
    }
    
    match numbers.get(10) {
        Some(value) => println!("Element at index 10: {}", value),
        None => println!("No element at index 10"),
    }
    
    println!("Third element: {}", numbers[2]);
}

fn exercise4_solution() {
    println!("=== Exercise 4: Vector Iteration ===");
    
    let mut scores = vec![85, 92, 78, 96, 88];
    
    for score in &scores {
        println!("Score: {}", score);
    }
    
    for (index, score) in scores.iter().enumerate() {
        println!("Score {}: {}", index + 1, score);
    }
    
    for score in &mut scores {
        *score += 5;
    }
    
    println!("Modified scores: {:?}", scores);
}

fn exercise5_solution() {
    println!("=== Exercise 5: Vector Methods ===");
    
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    
    println!("Length: {}", numbers.len());
    println!("Is empty: {}", numbers.is_empty());
    
    match numbers.iter().find(|&&x| x == 5) {
        Some(value) => println!("Found 5: {}", value),
        None => println!("5 not found"),
    }
    
    match numbers.iter().position(|&x| x == 9) {
        Some(pos) => println!("9 found at position: {}", pos),
        None => println!("9 not found"),
    }
    
    println!("Contains 7: {}", numbers.contains(&7));
    
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", evens);
    
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
}

fn exercise6_solution() {
    println!("=== Exercise 6: Sorting and Dedup ===");
    
    let mut names = vec!["Charlie", "Alice", "Bob", "Alice", "David", "Bob"];
    
    names.sort();
    println!("Sorted: {:?}", names);
    
    names.dedup();
    println!("Deduplicated: {:?}", names);
    
    names.sort_by(|a, b| b.cmp(a));
    println!("Reverse sorted: {:?}", names);
}

fn exercise7_solution() {
    println!("=== Exercise 7: Different Vector Types ===");
    
    let mut people = Vec::new();
    people.push(("Alice", 30));
    people.push(("Bob", 25));
    people.push(("Charlie", 35));
    
    if let Some(oldest) = people.iter().max_by_key(|&&(_, age)| age) {
        println!("Oldest person: {} ({})", oldest.0, oldest.1);
    }
    
    let names: Vec<&str> = people.iter().map(|(name, _)| *name).collect();
    println!("Names: {:?}", names);
    
    let bools = vec![true, false, true, true, false];
    let true_count = bools.iter().filter(|&&x| x).count();
    println!("True count: {}", true_count);
}

fn exercise8_solution() {
    println!("=== Exercise 8: Advanced Operations ===");
    
    let numbers1 = vec![1, 2, 3];
    let numbers2 = vec![4, 5, 6];
    
    let combined: Vec<i32> = numbers1.iter().chain(numbers2.iter()).cloned().collect();
    println!("Combined: {:?}", combined);
    
    let mut vec_2d = Vec::new();
    vec_2d.push(numbers1.clone());
    vec_2d.push(numbers2.clone());
    println!("2D vector: {:?}", vec_2d);
    
    let flattened: Vec<i32> = vec_2d.into_iter().flatten().collect();
    println!("Flattened: {:?}", flattened);
    
    let mixed = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (evens, odds): (Vec<i32>, Vec<i32>) = mixed.into_iter().partition(|&x| x % 2 == 0);
    println!("Evens: {:?}", evens);
    println!("Odds: {:?}", odds);
}
*/
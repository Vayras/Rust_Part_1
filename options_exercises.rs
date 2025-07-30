// Option Type - Custom Exercises
// Complete each exercise by replacing the TODO comments with working code

// Exercise 1: Creating and Basic Matching
fn exercise1() {
    println!("=== Exercise 1: Option Basics ===");
    
    // TODO: Create an Option<i32> with Some(42)
    
    // TODO: Create an Option<i32> with None
    
    // TODO: Create an Option<String> with Some("hello")
    
    // TODO: Use match to handle each Option and print appropriate messages
    
}

// Exercise 2: Working with Option Methods
fn exercise2() {
    println!("=== Exercise 2: Option Methods ===");
    
    let maybe_number = Some(10);
    let no_number: Option<i32> = None;
    
    // TODO: Use is_some() and is_none() to check the Options
    
    // TODO: Use unwrap_or() to provide default values
    
    // TODO: Use unwrap_or_else() with a closure to provide computed defaults
    
    // TODO: Use expect() on the Some value with a descriptive message
    
    // TODO: Demonstrate unwrap() on Some value (be careful!)
    
}

// Exercise 3: Transforming Options with map()
fn exercise3() {
    println!("=== Exercise 3: Option Transformations ===");
    
    let numbers = vec![Some(1), None, Some(3), Some(4), None];
    
    // TODO: Use map() to double all the Some values
    
    // TODO: Use filter() to keep only Some values greater than 2
    
    // TODO: Chain map operations to multiply by 2 then add 1
    
    // TODO: Print the results
    
}

// Exercise 4: Option with Collections
fn exercise4() {
    println!("=== Exercise 4: Options in Collections ===");
    
    let words = vec!["hello", "world", "rust", "programming"];
    
    // TODO: Use find() to search for a word that starts with 'r'
    
    // TODO: Use position() to find the index of "world"
    
    // TODO: Use get() to safely access vector elements
    
    // TODO: Handle all cases with proper Option matching
    
}

// Exercise 5: Chaining Options with and_then()
fn exercise5() {
    println!("=== Exercise 5: Option Chaining ===");
    
    // TODO: Create a function that returns Some(number) if positive, None if negative
    fn positive_only(n: i32) -> Option<i32> {
        // Your implementation here
    }
    
    // TODO: Create a function that returns Some(sqrt) if perfect square, None otherwise
    fn perfect_square_root(n: i32) -> Option<i32> {
        // Your implementation here
        // Hint: Use (n as f64).sqrt() and check if it's a whole number
    }
    
    // TODO: Chain these functions using and_then() to find positive perfect square roots
    
    // TODO: Test with various numbers: -4, 4, 9, 10, 16, 25
    
}

// Exercise 6: Converting Between Option and Result
fn exercise6() {
    println!("=== Exercise 6: Option and Result ===");
    
    let maybe_value = Some(42);
    let no_value: Option<i32> = None;
    
    // TODO: Convert Options to Results using ok_or()
    
    // TODO: Convert Results back to Options using ok()
    
    // TODO: Handle division by zero using Option
    fn safe_divide(a: f64, b: f64) -> Option<f64> {
        // Your implementation here
    }
    
    // TODO: Test safe_divide with different inputs
    
}

// Exercise 7: Real-world Option Usage
fn exercise7() {
    println!("=== Exercise 7: Real-world Examples ===");
    
    // TODO: Create a simple config struct with optional fields
    #[derive(Debug)]
    struct Config {
        host: String,
        port: Option<u16>,
        ssl: Option<bool>,
    }
    
    // TODO: Create a config with some optional fields as None
    
    // TODO: Create a function to get the effective port (default to 80 if None)
    
    // TODO: Create a function to check if SSL is enabled (default to false if None)
    
    // TODO: Print the config and effective values
    
}

// Exercise 8: Advanced Option Patterns
fn exercise8() {
    println!("=== Exercise 8: Advanced Patterns ===");
    
    // TODO: Use if let for cleaner Option handling
    let maybe_name = Some("Alice");
    // Handle with if let instead of match
    
    // TODO: Use while let to process a vector of Options until None is found
    let mut options = vec![Some(1), Some(2), Some(3), None, Some(4)];
    
    // TODO: Use Option::transpose() to transpose Vec<Option<T>> to Option<Vec<T>>
    let vec_of_options = vec![Some(1), Some(2), Some(3)];
    let vec_with_none = vec![Some(1), None, Some(3)];
    
    // TODO: Demonstrate the difference in transposing
    
}

// Exercise 9: Error Handling with Option
fn exercise9() {
    println!("=== Exercise 9: Error Handling ===");
    
    // TODO: Create a function that parses a string to a number, returning Option
    fn parse_number(s: &str) -> Option<i32> {
        // Use str::parse() and convert Result to Option
    }
    
    // TODO: Create a function that validates and processes user input
    fn process_input(input: &str) -> Option<String> {
        // Parse the input as a number, check if it's positive,
        // and return a formatted string if valid
    }
    
    // TODO: Test with valid and invalid inputs
    let test_inputs = vec!["42", "-5", "abc", "0", "100"];
    
}

// Exercise 10: Option Performance and Best Practices
fn exercise10() {
    println!("=== Exercise 10: Best Practices ===");
    
    // TODO: Demonstrate when to use unwrap vs expect vs unwrap_or
    let trusted_value = Some(42); // We know this has a value
    let user_input: Option<i32> = None; // This might be None
    let config_value: Option<i32> = None; // Should have a default
    
    // TODO: Show best practices for each scenario
    
    // TODO: Demonstrate Option::flatten() for nested Options
    let nested: Option<Option<i32>> = Some(Some(42));
    let also_nested: Option<Option<i32>> = Some(None);
    
    // TODO: Use flatten to simplify
    
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
    exercise9();
    exercise10();
    
    println!("\n🎉 All Option exercises completed!");
}

// Solutions (uncomment to check your work):
/*
fn exercise1_solution() {
    println!("=== Exercise 1: Option Basics ===");
    
    let some_number = Some(42);
    let no_number: Option<i32> = None;
    let some_string = Some(String::from("hello"));
    
    match some_number {
        Some(n) => println!("Got number: {}", n),
        None => println!("No number"),
    }
    
    match no_number {
        Some(n) => println!("Got number: {}", n),
        None => println!("No number found"),
    }
    
    match some_string {
        Some(s) => println!("Got string: {}", s),
        None => println!("No string"),
    }
}

fn exercise2_solution() {
    println!("=== Exercise 2: Option Methods ===");
    
    let maybe_number = Some(10);
    let no_number: Option<i32> = None;
    
    println!("maybe_number is_some: {}", maybe_number.is_some());
    println!("no_number is_none: {}", no_number.is_none());
    
    println!("maybe_number or 0: {}", maybe_number.unwrap_or(0));
    println!("no_number or 0: {}", no_number.unwrap_or(0));
    
    println!("no_number or computed: {}", no_number.unwrap_or_else(|| 5 * 5));
    
    println!("maybe_number expected: {}", maybe_number.expect("Should have a number"));
    println!("maybe_number unwrapped: {}", maybe_number.unwrap());
}

fn exercise3_solution() {
    println!("=== Exercise 3: Option Transformations ===");
    
    let numbers = vec![Some(1), None, Some(3), Some(4), None];
    
    let doubled: Vec<Option<i32>> = numbers.iter().map(|opt| opt.map(|x| x * 2)).collect();
    println!("Doubled: {:?}", doubled);
    
    let filtered: Vec<Option<i32>> = numbers.iter()
        .map(|opt| opt.filter(|&&x| x > 2))
        .collect();
    println!("Filtered > 2: {:?}", filtered);
    
    let chained: Vec<Option<i32>> = numbers.iter()
        .map(|opt| opt.map(|x| x * 2).map(|x| x + 1))
        .collect();
    println!("Chained operations: {:?}", chained);
}

fn exercise4_solution() {
    println!("=== Exercise 4: Options in Collections ===");
    
    let words = vec!["hello", "world", "rust", "programming"];
    
    match words.iter().find(|word| word.starts_with('r')) {
        Some(word) => println!("Found word starting with 'r': {}", word),
        None => println!("No word starting with 'r'"),
    }
    
    match words.iter().position(|&word| word == "world") {
        Some(pos) => println!("'world' found at position: {}", pos),
        None => println!("'world' not found"),
    }
    
    match words.get(2) {
        Some(word) => println!("Word at index 2: {}", word),
        None => println!("No word at index 2"),
    }
    
    match words.get(10) {
        Some(word) => println!("Word at index 10: {}", word),
        None => println!("No word at index 10"),
    }
}

fn positive_only_solution(n: i32) -> Option<i32> {
    if n > 0 {
        Some(n)
    } else {
        None
    }
}

fn perfect_square_root_solution(n: i32) -> Option<i32> {
    let sqrt = (n as f64).sqrt();
    let rounded = sqrt.round() as i32;
    if rounded * rounded == n {
        Some(rounded)
    } else {
        None
    }
}

fn exercise5_solution() {
    println!("=== Exercise 5: Option Chaining ===");
    
    let test_numbers = vec![-4, 4, 9, 10, 16, 25];
    
    for num in test_numbers {
        let result = positive_only_solution(num).and_then(perfect_square_root_solution);
        match result {
            Some(sqrt) => println!("{} -> positive perfect square root: {}", num, sqrt),
            None => println!("{} -> not a positive perfect square", num),
        }
    }
}

fn safe_divide_solution(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

fn exercise6_solution() {
    println!("=== Exercise 6: Option and Result ===");
    
    let maybe_value = Some(42);
    let no_value: Option<i32> = None;
    
    let result1: Result<i32, &str> = maybe_value.ok_or("No value found");
    let result2: Result<i32, &str> = no_value.ok_or("No value found");
    
    println!("Option to Result: {:?}, {:?}", result1, result2);
    
    let back_to_option1 = result1.ok();
    let back_to_option2 = result2.ok();
    
    println!("Result to Option: {:?}, {:?}", back_to_option1, back_to_option2);
    
    println!("10.0 / 2.0 = {:?}", safe_divide_solution(10.0, 2.0));
    println!("10.0 / 0.0 = {:?}", safe_divide_solution(10.0, 0.0));
}

#[derive(Debug)]
struct Config {
    host: String,
    port: Option<u16>,
    ssl: Option<bool>,
}

impl Config {
    fn effective_port(&self) -> u16 {
        self.port.unwrap_or(80)
    }
    
    fn ssl_enabled(&self) -> bool {
        self.ssl.unwrap_or(false)
    }
}

fn exercise7_solution() {
    println!("=== Exercise 7: Real-world Examples ===");
    
    let config = Config {
        host: "localhost".to_string(),
        port: Some(8080),
        ssl: None,
    };
    
    println!("Config: {:?}", config);
    println!("Effective port: {}", config.effective_port());
    println!("SSL enabled: {}", config.ssl_enabled());
    
    let minimal_config = Config {
        host: "example.com".to_string(),
        port: None,
        ssl: None,
    };
    
    println!("Minimal config: {:?}", minimal_config);
    println!("Effective port: {}", minimal_config.effective_port());
    println!("SSL enabled: {}", minimal_config.ssl_enabled());
}
*/
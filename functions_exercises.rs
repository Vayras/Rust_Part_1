// Functions - Custom Exercises
// Complete each exercise by replacing the TODO comments with working code

// Exercise 1: Basic Function Definition
// TODO: Create a function called 'greet' that takes no parameters and prints "Hello from a function!"

// TODO: Create a function called 'greet_person' that takes a name parameter and greets that person

// Exercise 2: Functions with Return Values
// TODO: Create a function called 'add' that takes two i32 parameters and returns their sum

// TODO: Create a function called 'multiply' that takes two f64 parameters and returns their product

// Exercise 3: Expression vs Statement
// TODO: Create a function called 'get_larger' that takes two i32s and returns the larger one
// Use an if expression (no semicolon on the last line)

// TODO: Create a function called 'calculate_area' that takes width and height (f64) 
// and returns the area. Use multiple statements and an expression for the return.

// Exercise 4: Early Returns
// TODO: Create a function called 'divide' that takes two f64 parameters
// Return early with 0.0 if the divisor is 0, otherwise return the division result

// TODO: Create a function called 'grade_letter' that takes a score (i32) and returns a &str
// Return "A" for 90+, "B" for 80+, "C" for 70+, "D" for 60+, "F" for below 60
// Use early returns for cleaner code

// Exercise 5: Functions Calling Functions
// TODO: Create a function called 'celsius_to_fahrenheit' that converts Celsius to Fahrenheit
// Formula: F = C * 9/5 + 32

// TODO: Create a function called 'fahrenheit_to_celsius' that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9

// TODO: Create a function called 'temperature_info' that takes a Celsius temperature
// and prints both Celsius and Fahrenheit values using the conversion functions

// Exercise 6: Complex Function Logic
// TODO: Create a function called 'is_leap_year' that takes a year (i32) and returns bool
// A leap year is divisible by 4, except century years must be divisible by 400

// TODO: Create a function called 'days_in_month' that takes month (i32) and year (i32)
// and returns the number of days in that month. Use is_leap_year for February.

fn main() {
    println!("=== Functions Exercises ===");
    
    println!("\n--- Exercise 1: Basic Functions ---");
    // TODO: Call your greet function
    // TODO: Call greet_person with your name
    
    println!("\n--- Exercise 2: Return Values ---");
    // TODO: Call add with 5 and 3, store result and print it
    // TODO: Call multiply with 2.5 and 4.0, store result and print it
    
    println!("\n--- Exercise 3: Expressions ---");
    // TODO: Call get_larger with 10 and 15, print the result
    // TODO: Call calculate_area with 5.0 and 3.0, print the result
    
    println!("\n--- Exercise 4: Early Returns ---");
    // TODO: Test divide with normal numbers and with zero divisor
    // TODO: Test grade_letter with different scores
    
    println!("\n--- Exercise 5: Function Composition ---");
    // TODO: Test temperature conversions
    // TODO: Call temperature_info with 25.0
    
    println!("\n--- Exercise 6: Complex Logic ---");
    // TODO: Test is_leap_year with 2020, 2021, 1900, 2000
    // TODO: Test days_in_month for February in leap and non-leap years
    
    println!("\nAll function exercises completed!");
}

// Solutions (uncomment to check your work):
/*
fn greet() {
    println!("Hello from a function!");
}

fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn get_larger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn calculate_area(width: f64, height: f64) -> f64 {
    let area = width * height;
    area
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }
    a / b
}

fn grade_letter(score: i32) -> &'static str {
    if score >= 90 {
        return "A";
    }
    if score >= 80 {
        return "B";
    }
    if score >= 70 {
        return "C";
    }
    if score >= 60 {
        return "D";
    }
    "F"
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn temperature_info(celsius: f64) {
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C is {}°F", celsius, fahrenheit);
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(month: i32, year: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0, // Invalid month
    }
}

fn main_solution() {
    println!("=== Functions Exercises ===");
    
    println!("\n--- Exercise 1: Basic Functions ---");
    greet();
    greet_person("Alice");
    
    println!("\n--- Exercise 2: Return Values ---");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    let product = multiply(2.5, 4.0);
    println!("2.5 * 4.0 = {}", product);
    
    println!("\n--- Exercise 3: Expressions ---");
    println!("Larger of 10 and 15: {}", get_larger(10, 15));
    println!("Area of 5.0 x 3.0: {}", calculate_area(5.0, 3.0));
    
    println!("\n--- Exercise 4: Early Returns ---");
    println!("10.0 / 2.0 = {}", divide(10.0, 2.0));
    println!("10.0 / 0.0 = {}", divide(10.0, 0.0));
    println!("Grade for 95: {}", grade_letter(95));
    println!("Grade for 75: {}", grade_letter(75));
    println!("Grade for 55: {}", grade_letter(55));
    
    println!("\n--- Exercise 5: Function Composition ---");
    temperature_info(25.0);
    println!("32°F is {}°C", fahrenheit_to_celsius(32.0));
    
    println!("\n--- Exercise 6: Complex Logic ---");
    println!("2020 is leap year: {}", is_leap_year(2020));
    println!("2021 is leap year: {}", is_leap_year(2021));
    println!("1900 is leap year: {}", is_leap_year(1900));
    println!("2000 is leap year: {}", is_leap_year(2000));
    
    println!("Days in Feb 2020: {}", days_in_month(2, 2020));
    println!("Days in Feb 2021: {}", days_in_month(2, 2021));
    println!("Days in April: {}", days_in_month(4, 2021));
}
*/
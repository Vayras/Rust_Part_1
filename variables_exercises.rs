// Variables and Mutability - Custom Exercises
// Complete each exercise by replacing the TODO comments with working code

// Exercise 1: Basic Variable Declaration
fn exercise1() {
    // TODO: Declare an immutable variable 'x' with value 10
    
    // TODO: Try to change x (this should cause a compiler error - comment it out after testing)
    // x = 20;
    
    // TODO: Declare a mutable variable 'y' with value 5
    
    // TODO: Change y to 15
    
    // TODO: Print both variables
    
}

// Exercise 2: Variable Shadowing
fn exercise2() {
    // TODO: Declare a variable 'name' with value "Alice"
    
    // TODO: Shadow 'name' with value "Bob"
    
    // TODO: Shadow 'name' again, but this time with the length of the string
    // Hint: use name.len()
    
    // TODO: Print the final value of name
    
}

// Exercise 3: Constants and Statics
// TODO: Declare a constant MAX_SCORE with value 100
// TODO: Declare a static GAME_NAME with value "Rust Adventure"

fn exercise3() {
    // TODO: Print both the constant and static variable
    
}

// Exercise 4: Type Annotations
fn exercise4() {
    // TODO: Declare a variable with explicit type annotation for i32
    
    // TODO: Declare a variable with explicit type annotation for f64
    
    // TODO: Declare a variable with explicit type annotation for bool
    
    // TODO: Declare a variable with explicit type annotation for char
    
    // TODO: Print all variables
    
}

// Exercise 5: Scope and Lifetime
fn exercise5() {
    let outer = "I'm in the outer scope";
    
    {
        // TODO: Declare a variable 'inner' that's only valid in this scope
        
        // TODO: Print both outer and inner
        
        // TODO: Shadow the outer variable with a new value
        
    } // inner goes out of scope here
    
    // TODO: Print outer (should be the original value)
    
    // TODO: Try to print inner (this should cause an error - comment it out)
    // println!("{}", inner);
}

// Exercise 6: Variable Patterns
fn exercise6() {
    // TODO: Use destructuring to assign multiple variables at once
    let tuple = (10, 20, 30);
    // let (?, ?, ?) = tuple;
    
    // TODO: Use underscore to ignore values you don't need
    let array = [1, 2, 3, 4, 5];
    // let [first, _, third, ..] = array;
    
    // TODO: Print the extracted values
    
}

fn main() {
    println!("=== Variables and Mutability Exercises ===");
    
    println!("\n--- Exercise 1: Basic Variables ---");
    exercise1();
    
    println!("\n--- Exercise 2: Shadowing ---");
    exercise2();
    
    println!("\n--- Exercise 3: Constants ---");
    exercise3();
    
    println!("\n--- Exercise 4: Type Annotations ---");
    exercise4();
    
    println!("\n--- Exercise 5: Scope ---");
    exercise5();
    
    println!("\n--- Exercise 6: Patterns ---");
    exercise6();
    
    println!("\nAll exercises completed!");
}

// Solutions (uncomment to check your work):
/*
const MAX_SCORE: u32 = 100;
static GAME_NAME: &str = "Rust Adventure";

fn exercise1_solution() {
    let x = 10;
    // x = 20; // This would cause an error
    let mut y = 5;
    y = 15;
    println!("x: {}, y: {}", x, y);
}

fn exercise2_solution() {
    let name = "Alice";
    let name = "Bob";
    let name = name.len();
    println!("name: {}", name);
}

fn exercise3_solution() {
    println!("Max score: {}, Game: {}", MAX_SCORE, GAME_NAME);
}

fn exercise4_solution() {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';
    println!("int: {}, float: {}, bool: {}, char: {}", integer, float, boolean, character);
}

fn exercise5_solution() {
    let outer = "I'm in the outer scope";
    {
        let inner = "I'm in the inner scope";
        println!("Outer: {}, Inner: {}", outer, inner);
        let outer = "I'm shadowing the outer variable";
        println!("Shadowed outer: {}", outer);
    }
    println!("Original outer: {}", outer);
}

fn exercise6_solution() {
    let tuple = (10, 20, 30);
    let (first, second, third) = tuple;
    
    let array = [1, 2, 3, 4, 5];
    let [a, _, c, ..] = array;
    
    println!("Tuple: {}, {}, {}", first, second, third);
    println!("Array: {}, {}", a, c);
}
*/
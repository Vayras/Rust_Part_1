// Introduction to Rust - Custom Exercises
// Complete each exercise by replacing the TODO comments with working code

// Exercise 1: Hello World Variations
fn exercise1() {
    // TODO: Print "Hello, Rust!" to the console
    
    // TODO: Print your name with a greeting
    
    // TODO: Print multiple lines using println! multiple times
    
}

// Exercise 2: Basic Syntax
fn exercise2() {
    // TODO: Create a main function that calls a helper function
    
    // TODO: The helper function should print "Learning Rust is fun!"
    
}

// Exercise 3: Comments and Documentation
fn exercise3() {
    // TODO: Add a single-line comment explaining what this function does
    
    /* TODO: Add a multi-line comment 
       explaining the purpose of this exercise */
    
    /// TODO: Add a documentation comment for this function
    fn documented_function() {
        println!("This function is documented!");
    }
    
    documented_function();
}

// Exercise 4: Cargo Basics
// TODO: Create a new Cargo project called "my_first_rust_project"
// TODO: Add a dependency to Cargo.toml (you can use `rand = "0.8"`)
// TODO: Use the dependency in your main.rs file

// Exercise 5: Compilation and Running
// TODO: Compile this file using rustc
// TODO: Run the compiled binary
// TODO: Use cargo check to verify your code compiles
// TODO: Use cargo run to compile and run in one step

fn main() {
    println!("Starting Rust exercises!");
    
    exercise1();
    exercise2();
    exercise3();
    
    println!("Congratulations! You've completed the intro exercises!");
}

// Solutions (uncomment to check your work):
/*
fn exercise1_solution() {
    println!("Hello, Rust!");
    println!("Hello, Alice!");
    println!("Welcome to Rust programming!");
    println!("Let's learn together!");
}

fn exercise2_solution() {
    fn helper() {
        println!("Learning Rust is fun!");
    }
    helper();
}

fn exercise3_solution() {
    // This function demonstrates different types of comments
    
    /* This is a multi-line comment
       that spans multiple lines */
    
    /// This function shows how documentation comments work
    /// It can be used by rustdoc to generate documentation
    fn documented_function() {
        println!("This function is documented!");
    }
    
    documented_function();
}
*/
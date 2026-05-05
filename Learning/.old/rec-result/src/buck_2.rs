use std::fs::File;
use std::error::Error;

// ❌ This WON'T compile - main returns (), not Result
fn main_that_wont_compile() {
    let file = File::open("hello.txt")?; // ERROR!
}

// ✅ This WILL compile - main returns Result
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("hello.txt")?; // OK!
    println!("File opened successfully!");
    Ok(()) // Must return Ok(()) for success
}

// ✅ Also works with custom functions
fn my_function() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("data.txt")?; // OK!
    Ok(contents)
}

// Example with Option<T>
fn find_last_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() // ? works with Option too!
}

fn test_examples() {
    // Testing the Option example
    match find_last_char("Hello\nWorld") {
        Some(ch) => println!("Last char of first line: {}", ch),
        None => println!("No characters found"),
    }
    
    // Testing our custom function
    match my_function() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
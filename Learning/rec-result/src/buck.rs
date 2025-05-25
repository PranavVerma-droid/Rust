use std::fs::File;
use std::io::{self, Read};

// Same function as before, but MUCH shorter with ?
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // ? handles the error
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // ? handles the error
    Ok(username)
}

// Even shorter - method chaining with ?
fn read_username_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Shortest of all - using built-in function
fn read_username_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

fn main() {
    println!("=== Method 1 ===");
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("=== Method 2 (shorter) ===");
    match read_username_shorter() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("=== Method 3 (shortest) ===");
    match read_username_shortest() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }
}
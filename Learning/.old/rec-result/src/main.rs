use std::fs::File;
use std::io::ErrorKind;

/*
fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => {
            println!("File opened successfully!");
            file
        },
        Err(error) => {
            // Check what KIND of error occurred
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("File doesn't exist. Let's create it!");
                    // Try to create the file
                    match File::create("hello.txt") {
                        Ok(new_file) => {
                            println!("Created new file successfully!");
                            new_file
                        },
                        Err(create_error) => {
                            panic!("Couldn't create file: {}", create_error);
                        }
                    }
                },
                _ => {
                    // Any other error (permission denied, etc.)
                    panic!("Problem opening file: {}", error);
                }
            }
        }
    };
    
    println!("We now have a file handle to work with!");
}
*/

// Writing all those match statements gets tedious. Rust provides shortcuts when you just want to panic if something goes wrong:
use std::fs::File;

fn main() {
    // Using unwrap() - panics with default message if error
    println!("=== Using unwrap() ===");
    let file1 = File::open("hello.txt").unwrap();
    // If file doesn't exist: panic with generic message
    
    // Using expect() - panics with YOUR custom message if error
    println!("=== Using expect() ===");
    let file2 = File::open("hello.txt")
        .expect("hello.txt should exist in this project");
    // If file doesn't exist: panic with YOUR message
    
    println!("Both files opened successfully!");
}

// Example of what the panic messages look like:

// unwrap() panic message:
// thread 'main' panicked at src/main.rs:6:47:
// called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

// expect() panic message:  
// thread 'main' panicked at src/main.rs:11:10:
// hello.txt should exist in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }

/* unwrap(): Quick prototyping, when you're absolutely sure it won't fail
expect(): Better for production code - gives meaningful error messages
match: When you want to handle errors gracefully instead of panicking */

// This is how Result is defined in Rust
enum Result<T, E> {
    Ok(T),    // Success case - contains the successful value
    Err(E),   // Error case - contains error information
}

// Example: Opening a file
// T = std::fs::File (the file handle if successful)
// E = std::io::Error (error info if it fails)

/* Ok(value) - "Success! Here's what you wanted"
Err(error) - "Sorry, something went wrong. Here's what happened" */
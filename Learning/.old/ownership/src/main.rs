fn main() {
    // String literals are fixed, immutable, and stored in the program binary
    let s1 = "hello"; // This is a &str, not a String

    // String type is mutable and heap-allocated
    let mut s2 = String::from(s1);
    s2.push_str(", world!"); // Can modify it
    println!("{}", s2); // Prints "hello, world!"

    let s1 = String::from("hello");

    //let s2 = s1; // Not a copy - this is a move!
    // println!("{}", s1); // Error! s1 is no longer valid

    let s2 = s1.clone(); // Explicitly duplicates heap data
    println!("s1 = {}, s2 = {}", s1, s2); // Both valid

    let x = 5;
    let y = x; // x is still valid because integers implement Copy
    println!("x = {}, y = {}", x, y); // Works fine

    /*
    Types that implement the Copy trait get copied instead of moved. These include:
      - All integer types (i32, u64, etc.)
      - Boolean (bool)
      - Floating point types (f64, etc.)
      - Characters (char)
      - Tuples containing only Copy types
     */

    let s = String::from("hello");
    takes_ownership(s); // s is moved into the function
    // println!("{}", s); // Error! s is no longer valid
     
    let x = 5;
    makes_copy(x); // x is copied, not moved
    println!("{}", x); // Still valid

    let s1 = gives_ownership(); // Receives ownership from function
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 moved into function,
                                       // and function returns ownership to s3

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens

fn gives_ownership() -> String {
    let s = String::from("yours");
    s // Return value moves ownership to calling function
}

fn takes_and_gives_back(s: String) -> String {
    s // Returns ownership to caller
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return both the string and its length
}

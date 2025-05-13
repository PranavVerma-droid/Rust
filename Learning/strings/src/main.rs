fn main() {
    /*
    In Rust, working with text is handled through two main types:

     -   &str - a string slice (borrowed, fixed-size view into text)
     -   String - an owned, growable text type    
    */
    
    /* 
    String slices (&str) - These are references to UTF-8 encoded string data stored elsewhere. 
    String literals (text in double quotes like "hello") are stored in the program's binary and are &str type.

    String type (String) - This is provided by Rust's standard library. It's growable, mutable, owned, and UTF-8 encoded.
    */

    // String literal - a &str type
    let slice = "I am a string slice";

    // Creating a String from a string slice
    let string = String::from("I am a String");

    // Creating an empty String
    let mut empty_string = String::new();

    // Creating a String from a string literal using to_string()
    let s1 = "initial contents".to_string();

    // Creating a String from a string literal using String::from()
    let s2 = String::from("initial contents");

    let hello_english = String::from("Hello");
    let hello_spanish = String::from("Hola");
    let hello_arabic = String::from("السلام عليكم");
    let hello_hebrew = String::from("שלום");
    let hello_hindi = String::from("नमस्ते");
    let hello_japanese = String::from("こんにちは");

    // You can add content to strings using methods like push_str() and push():

    // Adding a string slice to a String
    let mut s = String::from("foo");
    s.push_str("bar"); // s now contains "foobar"

    // Adding a single character to a String
    let mut s = String::from("lo");
    s.push('l'); // s now contains "lol"

    // Note: push_str() takes a string slice and doesn't take ownership of the parameter:

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is still usable: {s2}"); // Works fine!

    // You can combine strings using the + operator or the format! macro:

    // Using + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 is moved and can no longer be used

    /* 
    Important things to understand about the + operator:

    - It takes ownership of the first string (s1)
    - It only borrows the second string (&s2)
    - After the operation, s1 is no longer valid, but s2 is still usable
    */

    // For combining multiple strings, the format! macro is more convenient:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Using + operator (gets unwieldy)
    let s = s1 + "-" + &s2 + "-" + &s3; // s1 is moved

    // Using format! (cleaner and doesn't move any values)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");  // All variables still usable

    // Unlike many programming languages, Rust doesn't allow you to access characters in a string by index:

    let s = String::from("hello");
    // let h = s[0]; => This will NOT compile in Rust

    // This restriction exists because of how Rust stores strings in memory (as UTF-8 encoded bytes) and because characters in UTF-8 can take different amounts of memory.

    // A String is essentially a wrapper around a Vec<u8> (vector of bytes):
    let hello = String::from("Hola"); // 4 bytes in UTF-8
    let hello = String::from("Здравствуйте"); // 24 bytes in UTF-8

    /* 
    In the second example, even though there are 12 characters, the string takes 24 bytes because each Cyrillic character takes 2 bytes in UTF-8.
    If Rust allowed direct indexing like hello[0]:

    For ASCII text, you might expect the first character
    But for multi-byte characters, you'd get a byte that might not even be a valid character on its own
    */

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Gets the first 4 bytes, which is "Зд"

    // Warning: This can panic if you slice in the middle of a character.

    // Iterating over Unicode scalar values (chars)
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Prints:
    // З
    // д

    // Iterating over raw bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // Prints:
    // 208
    // 151
    // 208
    // 180
}
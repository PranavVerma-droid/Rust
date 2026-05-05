fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);
    
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello"); // Note the 'mut' here
    
    change(&mut s); // And '&mut' here



    let mut s = String::from("hello world");
    
    let _word = first_word(&s); // word gets the value 5
    
    s.clear(); // This empties the string
    
    /* 
    Word still has value 5, but it's now meaningless!
    This code compiles without errors, but it's a logical error waiting to happen. 
    The number 5 is no longer valid as an index into the string after s.clear() is called, 
    but the compiler has no way to detect this.
     */

    let s = String::from("hello world");

    let hello = &s[0..5];  // A slice containing "hello"
    let world = &s[6..11]; // A slice containing "world"
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) { // And '&mut' here
    some_string.push_str(", world"); // Now this works!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn main() {
    /* A panic is Rust's way of saying "Something went so wrong that I can't continue safely." 
    It's like a controlled crash - instead of your program doing unpredictable things, 
    Rust stops everything and tells you exactly what went wrong. */

    // Basic Example of a Panic Program:
    println!("Starting program...");
    // panic!("Oh no! Something terrible happened!");
    // println!("This line will never execute");

    // The program stops immediately when it hits panic! - that last println! never runs.

    // Create a vector with 3 elements (indexes 0, 1, 2)
    let numbers = vec![10, 20, 30];
    
    println!("The vector has {} elements", numbers.len());
    
    // This is fine - index 1 exists
    println!("Second element: {}", numbers[1]);
    
    // This will panic! - index 99 doesn't exist
    // println!("Element at index 99: {}", numbers[99]);

    // In languages like C, accessing memory you shouldn't would just give you random garbage data or crash unpredictably. Rust prevents this by checking bounds and panicking safely instead.

    call_function_a();
}


fn call_function_a() {
    println!("In function A");
    call_function_b();
}

fn call_function_b() {
    println!("In function B");
    // This will panic
    let data = vec![1, 2, 3];
    let bad_access = data[100]; // Panic here!
}


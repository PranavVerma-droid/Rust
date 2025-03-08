fn main() {
    println!("Hello, world!");
    
    my_function();

    // Passing the argument 5 to the function
    print_number(5);
    
    // Passing multiple arguments
    print_values(10, 'x');


    let y = 6;
    
    // This is an expression in a statement
    let z = {
        let x = 3;
        x + 1  // Note: no semicolon here!
    };
    
    println!("z is: {z}");  // This will print "z is: 4"

    let x = five();
    println!("The value of x is: {x}");  // Prints "The value of x is: 5"
    
    let y = plus_one(5);
    println!("The value of y is: {y}");  // Prints "The value of y is: 6"

    let number = {
        let temp = 3;
        temp * 2  // This value (6) is returned from the block
    };
    
    println!("Number: {number}");  // Prints "Number: 6"
}

fn my_function() {
    println!("This is my function!");
}

// Function with one parameter
fn print_number(x: i32) {
    println!("The number is: {x}");
}

// Function with multiple parameters
fn print_values(value: i32, label: char) {
    println!("Value: {value}{label}");
}

fn five() -> i32 {
    5  // No semicolon - this is an expression that returns 5
}

fn plus_one(x: i32) -> i32 {
    x + 1  // Returns x + 1 (no semicolon!)
}
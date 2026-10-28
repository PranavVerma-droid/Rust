mod loops;                   // Declare 
use loops::{loops, loops_2, loops_3}; // Import Functions from `loops.rs` File.

fn main() {
    let number: i32 = 10;


    // Unlike Python, this won't work:
    // if number {
    if number < 10 {
        println!("Less than 10.");
    } else {
        println!("More than or equal to 10.");
    }

    // Multiple conditions are handeled with `else if`, not elif

    if number < 10 {
        // if LESS than 10
    } else if number == 10 {
        // if EQUAL to 10
    } else {
        // everything else (greater than 10)
    }

    let condition = true;
    let num2 = if condition { 5 } else {
        6
    };

    // We cannot do this:
    // let number = if condition { 5 } else { "six" };
    // Rust expects int type in the else aswell, or else panic.

    println!("The value of num2 is {num2}");


    // Loops: (check loops.rs)
    loops();
    loops_2();
    loops_3();

}

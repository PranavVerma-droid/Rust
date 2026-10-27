fn main() {
    println!("Hello, world!");
    
    another_function(5);
    print_labelled_measurements(5, 'h');
    print_labelled_measurements_long(5, "meters");

    // Notice the misisng `;` inside the y scope.
    // If you add a `;` to it, you turn it to a statement, giving you a error.
    // This will return 3, so Value of y will be `3`.
    let _y = {
        let x = 1;
        x + 2
    };

    let _z = five();
    let _a = {five()};
}

// Defined using `fn`.
fn another_function(x: isize) { // The Stuff inside the bracket are `parameters`
    println!("Another Function.");
    println!("The value of x is {x}")
}

fn print_labelled_measurements(x: isize, unit: char) {
    println!("The value of x is {x} {unit}");
}

fn print_labelled_measurements_long(x: isize, unit: &str) {
    println!("The value of x is {x} {unit}");
}

fn five() -> i32 { // This Arrow `->` means the return value.
    5 // Again, if we use `5;` then we turn it into a statement.
}

fn plus_one(x: i32) -> i32 {
    (x + 1)
}
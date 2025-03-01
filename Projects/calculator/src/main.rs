use std::io;

fn add(x:i128, y:i128) -> f32 {
    return (x + y) as f32;
}

fn subtract(x:i128, y:i128) -> f32 {
    return (x - y) as f32;
}

fn multiply(x:i128, y:i128) -> f32 {
    return (x * y) as f32;
}

fn divide(x:i128, y:i128) -> f32 {
    return (x / y) as f32;
}

fn main() {
    println!("Functions:");
    println!("Enter 1 to Add.");
    println!("Enter 2 to Subtract.");
    println!("Enter 3 to Multiply.");
    println!("Enter 4 to Divide.");
    println!("");
    println!("Enter 0 to Exit.");
    println!("");

    let mut guess_str: String = String::new();
    let mut num1_str = String::new();
    let mut num2_str = String::new();

    println!("Please Enter:");
    let _ = io::stdin().read_line(&mut guess_str);
    let guess: f32 = guess_str.trim().parse().expect("Please enter a valid number");

    println!("Please Enter Num 1:");
    let _ = io::stdin().read_line(&mut num1_str);
    let num1: f32 = num1_str.trim().parse().expect("Please enter a valid number");

    println!("Please Enter Num 2:");
    let _ = io::stdin().read_line(&mut num2_str);
    let num2: f32 = num2_str.trim().parse().expect("Please enter a valid number");

    if (guess == 1 as f32) {
        println!("Result: {}", add(num1 as i128, num2 as i128));
        main();
    } else if ((guess == 2 as f32)) {
        println!("Result: {}", subtract(num1 as i128, num2 as i128));
        main();
    } else if ((guess == 3 as f32)) {
        println!("Result: {}", multiply(num1 as i128, num2 as i128));
        main();
    } else if ((guess == 4 as f32)) {
        println!("Result: {}", divide(num1 as i128, num2 as i128));
        main();
    } else if ((guess == 0 as f32)) {
        
    } else{
        println!("Please enter valid operator.");
        main();
    }


}


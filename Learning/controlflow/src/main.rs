fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*  This will NOT compile
    if number {  // Error: expected bool, found integer
    println!("number exists"); 
    } */

    // Instead, you need to be explicit:
    if number != 0 {
        println!("number is not zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}"); // Prints: The value of number is: 5
    /* This will NOT compile
    let number = if condition { 5 } else { "six" }; // Error: `if` and `else` have incompatible types */


    let mut counter = 0;
    
    loop {
        println!("Count: {counter}");
        counter += 1;
        
        if counter >= 10 {
            break; // Exit the loop when counter reaches 10
        }
    }
    println!("Loop finished!");



    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Return counter * 2 from the loop
        }
    };

    println!("The result is {result}"); // Prints: The result is 20


    let mut count = 0;
    
    'counting_up: loop {          // This outer loop has a label 'counting_up
        println!("count = {count}");
        let mut remaining = 10;

        loop {                     // This is the inner loop
            println!("remaining = {remaining}");
            
            if remaining == 9 {
                break;            // This breaks the inner loop only
            }
            
            if count == 2 {
                break 'counting_up; // This breaks the outer loop
            }
            
            remaining -= 1;
        }

        count += 1;
    }
    
    println!("End count = {count}");


    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    // Using a for loop to iterate through array elements
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


    for i in 1..=20 {  // Note: 1..=20 includes 20
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }

    findValue();
}


fn findValue() {
    let numbers = [10, 20, 30, 40, 50];
    let search_for = 30;
    let mut found = false;
    
    for &num in &numbers {
        if num == search_for {
            found = true;
            break;
        }
    }
    
    if found {
        println!("Found {search_for}!");
    } else {
        println!("{search_for} not found in the array.");
    }
}
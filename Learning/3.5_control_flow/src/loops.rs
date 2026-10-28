pub fn loops() {
    println!("Test: Imports work");

    loop {
        println!("Testing loop");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter = counter + 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The Result is {result}");
}

pub fn loops_2() {
    let mut count = 0;

    'counting_up: loop { // Apply a Label to Loops
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            
            if count == 2 {
                break 'counting_up; 
                // You can break the outermost loop directly aswell.
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}

pub fn loops_3() {
    let mut number = 3;


    // Using `while`:
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Using `for` to access elements in an iterable
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Nicer Version using `for`:
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
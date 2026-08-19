const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x + 2;
        println!("The value of x now is: {x}");
    }

    println!("The Value of x Finally is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    // This code will not work:
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
     */

}

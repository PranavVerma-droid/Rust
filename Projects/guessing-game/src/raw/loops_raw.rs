fn main() {
    // Previous setup code...

    loop {
        println!("Please input your guess.");

        // Code to get and process the guess...

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
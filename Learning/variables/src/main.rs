fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // .parse() needs a type

    let decimal = 98_222;     // Decimal (underscores for readability)
    let hex = 0xff;           // Hexadecimal
    let octal = 0o77;         // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A';          // Byte (u8 only)

    // Example of wrapping_add: wraps around on overflow
    let max_u8 = u8::MAX;
    println!("wrapping_add: {} + 1 = {}", max_u8, max_u8.wrapping_add(1)); // Will wrap to 0

    // Example of checked_add: returns None on overflow
    let check_result = max_u8.checked_add(1);
    println!("checked_add: {} + 1 = {:?}", max_u8, check_result); // Will be None

    // Example of overflowing_add: returns the wrapped value and a boolean
    let (overflow_result, did_overflow) = max_u8.overflowing_add(1);
    println!("overflowing_add: {} + 1 = {}, did overflow: {}", max_u8, overflow_result, did_overflow);

    // Example of saturating_add: stops at the maximum value
    let saturated = max_u8.saturating_add(1);
    println!("saturating_add: {} + 1 = {}", max_u8, saturated); // Will be u8::
    
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let x = 2.0;      // f64 by default
    let y: f32 = 3.0; // f32 explicitly

    let t = true;
    let f: bool = false;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring
    let (x, y, z) = tup;
    println!("y is: {y}"); // Prints: y is: 6.4

    // Direct indexing
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // Equivalent to [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    // let element = a[10]; // This will cause a runtime panic
}

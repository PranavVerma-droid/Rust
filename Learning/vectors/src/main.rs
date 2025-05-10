fn main() {
    /* In Rust, a vector (Vec<T>) is like a growable array that can hold 
    multiple values of the same type. Think of it as a container that 
    can expand or shrink as needed, storing your data in a sequential list.*/

    // Creating a Vector
    /* Method 1: Using Vec::new()
    
    Notice the Vec<i32> part - this tells Rust what type of elements this vector 
    will hold (in this case, 32-bit integers). When you create an empty vector, 
    Rust can't infer what type it will contain, so you need to specify this. */
    let v: Vec<i32> = Vec::new();


    /* Method 2: Using the vec! macro

    This is much simpler! Rust can figure out that you want a Vec<i32> because you 
    provided numbers. The vec! macro creates a new vector with the values you give it. */
    let v = vec![1, 2, 3];

    /* Modifying a Vector 
    The push method adds new elements to the end of the vector. Notice we made the v
    ector mutable with mut - in Rust, variables are immutable (can't be changed) by default.*/
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    // Accessing Elements
    // Method 1: Using index syntax &v[index]
    let v = vec![1, 2, 3, 4, 5];

    // Get the third element (index 2, since counting starts at 0)
    let third = &v[2];
    println!("The third element is {third}");  // Prints: The third element is 3

    // Method 2: Using the get method
    let v = vec![1, 2, 3, 4, 5];

    // Try to get the third element
    let third = v.get(2);
    match third {
        Some(value) => println!("The third element is {value}"),
        None => println!("There is no third element."),
    }

    /* Here's a key difference between these methods:

    - With &v[index], if you try to access an element that doesn't exist, your program will crash (panic)
    - With v.get(index), you get an Option<&T>:
        Some(&value) if the element exists
        None if it doesn't exist */
}   

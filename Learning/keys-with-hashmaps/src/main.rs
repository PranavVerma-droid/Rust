use std::collections::HashMap;

fn main() {
    // Create an empty hash map
    let mut scores = HashMap::new();

    // Add key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /* 
    Hash maps follow Rust's ownership rules:

    - For types that implement the Copy trait (like i32), values are copied into the hash map
    - For types that don't implement Copy (like String), the hash map takes ownership of the values
    */

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are no longer valid here!

    /* 
    After inserting, you can't use field_name or field_value anymore because they've been "moved" into the hash map.
    If you inserted references instead, the values wouldn't be moved, but you'd need to make sure the referenced data lives at least as long as the hash map.
    */

        let text = "rust programming language is amazing rust is fast";
    
    // Count word frequencies
    let mut word_counts = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    
    // Print all words and their counts
    println!("Word frequencies:");
    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
    
    // Find the most common word
    let mut most_common_word = "";
    let mut highest_count = 0;
    
    for (word, &count) in &word_counts {
        if count > highest_count {
            most_common_word = word;
            highest_count = count;
        }
    }
    
    println!("\nMost common word: '{}' (appears {} times)", 
             most_common_word, highest_count);
}

// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure. Use when you need to modify or own string data.

pub fn run() {
    let hello_fixed = "hello"; // String, fixed-length
    let mut hello_growable = String::from("Hello"); // String, growable

    println!("Length of {} is {}", hello_fixed, hello_fixed.len());
    println!("Length of {} is {}", hello_growable, hello_growable.len());

    hello_growable.push(' '); // Add a character

    println!("Length of {} is {}", hello_growable, hello_growable.len());

    hello_growable.push_str(" World!"); // Add a string

    println!("Length of {} is {}", hello_growable, hello_growable.len());

    // Capacity in bytes
    println!("Capacity: {}", hello_growable.capacity());

    // Check if empty
    println!("Is empty: {}", hello_growable.is_empty());

    // Contains
    println!("Contains 'World': {}", hello_growable.contains("World"));

    // Replace
    println!("Replace: {}", hello_growable.replace("World", "Cunts"));

    // Loop through string by whitespace
    for word in hello_growable.split_whitespace() {
        println!("{}", word);
    }
}

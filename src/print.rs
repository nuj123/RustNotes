pub fn run() {
    // Print to console
    println!("Hello from print.rs file");

    // Basic formatting
    println!("Number: {}", 69);

    // More Basic formatting
    println!("{} is from {}", "Pyro", "Scotland");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes {2}",
        "Pyro", "Scotland", "programming"
    );

    // Named Arguments
    println!(
        "{name} likes the programming languages {langs:?}",
        name = "Pyro",
        langs = ["Assembly", "C++", "Python", "Rust"]
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}

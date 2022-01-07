// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Pyro";
    let mut age = 20;

    println!("My name is {} and I'm {}", name, age);
    age = 21;
    println!("My name is {} and I'm {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {:03}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Pyro", 21);
    println!("{} is {}", my_name, my_age);
}

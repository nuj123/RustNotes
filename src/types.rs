/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean: true, false
Characters: char (4 bytes)
Tuples: Tuple structs can be used as patterns
Arrays: Array structs can be used as patterns
*/

// Rust is a statically typed language, which means you have to specify the type of data you want to use.

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 635476234532;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128 {}", std::i128::MAX);

    // Boolean
    let is_active: bool = true;

    // Char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, a1, face));
}

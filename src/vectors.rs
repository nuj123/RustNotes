// Vectors are resizable arrays of values.
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Print array
    println!("{:?}", numbers);

    // Add value
    numbers.push(6);
    println!("{:?}", numbers);
    numbers.pop(); // Pop last value

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate
    for x in numbers.iter_mut() {
        *x <<= 1;
    }
    println!("Numbers Vec: {:?}", numbers);
}

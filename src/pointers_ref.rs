// Reference Pointers - Points to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;

    println!("Array Values: {:?}", (arr1, arr2));

    // Vector
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = &vec1;

    println!("Vector Values: {:?}", (&vec1, vec2));
}

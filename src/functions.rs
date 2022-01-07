// Funtions - Used to store a block of code that can be called later

pub fn run() {
    greet("Hello", "Pyro");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));

    // Default
    println!("D Sum 3,3: {}", add_nums_default(Some(3), Some(3)));
    println!("D Sum 3,N: {}", add_nums_default(Some(3), None));
    println!("D Sum N,N: {}", add_nums_default(None, None));
}

fn greet(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Default function parameters
fn add_nums_default(x: Option<i32>, y: Option<i32>) -> i32 {
    x.unwrap_or(10) + y.unwrap_or(20)
}

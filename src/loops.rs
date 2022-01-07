// Loops - Used to iterare until a condition is met

pub fn run() {
    let mut count = 0;

    //Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);
        if count == 20 {
            break;
        }
    }

    //While loop
    count = 1;
    let mut fizzbuzz_string;
    while count <= 100 {
        fizzbuzz_string = String::from("");
        if count % 3 == 0 { fizzbuzz_string.push_str("Fizz"); }
        if count % 5 == 0 { fizzbuzz_string.push_str("Buzz"); }
        if fizzbuzz_string.is_empty() { fizzbuzz_string = count.to_string(); }
        println!("{}", fizzbuzz_string);
        count += 1;
    }

    //For loop
    for x in 0..100 {
        fizzbuzz_string = String::from("");
        if x % 3 == 0 { fizzbuzz_string.push_str("Fizz"); }
        if x % 5 == 0 { fizzbuzz_string.push_str("Buzz"); }
        if fizzbuzz_string.is_empty() { fizzbuzz_string = x.to_string(); }
        println!("{}", fizzbuzz_string);
    }
}

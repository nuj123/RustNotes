pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("No arguments supplied");
    } else if args.len() == 2 || args[1] == "add" {
        match args[1].as_ref() {
            "hello" => println!("Hello World!"),
            "add" => println!(
                "{} + {} = {}",
                args[2],
                args[3],
                args[2].parse::<i32>().unwrap() + args[3].parse::<i32>().unwrap()
            ),
            _ => println!("Invalid argument"),
        }
    } else {
        for arg in args[1..].iter() {
            println!("Arg: {}", arg);
        }
    }
}

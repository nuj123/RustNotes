// Structs - Structs are a way to group data together to form a new data type.
// Traditional struct
struct Person {
    first_name: String,
    last_name: String,
}

// Tuple struct
struct Color2(u8, u8, u8);

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color2(255, 0, 0);
    c.0 = 200;
    println!("Color2: {} {} {}", c.0, c.1, c.2);

    let p = Person::new("Pyro", "DeathAdder");
    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple());
}

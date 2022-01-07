// Tuples group together values of different types.
// Max 12 elements in a tuple.

pub fn run() {
    let person: (&str, &str, i8) = ("Pyro", "Scotland", 21);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}

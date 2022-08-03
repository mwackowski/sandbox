// max 12 elements in tuple!

pub fn run() {
    let person: (&str, &str, i8) = ("wladzioo", "Poland", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
pub fn run() {
    let age: u8 = 22; 
    let check_id: bool = true;
    let knows_persons_age = true;


    if age >= 21 && (check_id || knows_persons_age) {
        println!("Bartender: what would you like to drink?");
    } else if age < 21 && check_id {
        println!("You're not old enough");
    } else {
        println!("Bartender: I'll need to see your id");
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age: {}", is_of_age);
}
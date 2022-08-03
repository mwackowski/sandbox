// Primitive str = immutable
// String = growable, heap-allocated data structure

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length 
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains 
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace 
    println!("Replace: {}", hello.replace("World", "there"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", hello);

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    
    println!("{}", s);



}
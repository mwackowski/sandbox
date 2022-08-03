pub fn run() {
    // Print to console
    println!("hello from the print.rs file");

    // Formatting
    println!("number: {}", 1);
    println!("{} is from {}", "wladzioo", "Poland");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "wladzioo", "Poland", "code");

    // Named arguments
    println!("{name} likes to play {activity}", 
    name="John", 
    activity="football"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traid
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10+10);
}
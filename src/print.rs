pub fn run() {
    // Print to console
    println!("Hello from the print.rs file yo");

    // Basic formating
    println!("Number {}", 1);
    println!("{} of {}", "Queen", "England");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Queen", "England", "code"
    );

    // Named Argumends
    println!("{name} likes to play {activity}", name = "King", activity = "Crazy");

    // Placeholder traits
    println!("Binary: {:b} Hex {:x} Octal {:o}", 105, 105, 105);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}

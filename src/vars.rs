// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Kong";
    let mut age = 32;

    println!("My name is {} and I am {} yearsy old", name, age);

    age = 33;

    println!(
        "My name is {} and I am {} yearsy old, fuck... :)",
        name, age
    );

    // Define constant. You have explicitly define a type
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("King", 100);

    println!(
        "I am {} and I have {} years spent in life yo",
        my_name, my_age
    );
}

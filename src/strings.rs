// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {

    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // for chars
    hello.push('Y');

    // for strings
    hello.push_str("es Man");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if its empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains: {}", hello.contains("Yes"));

    // Replace
    println!("Replace: {}", hello.replace("Yes Man", "Worlds!"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    };

    // Create String with capacity
    let mut s = String::with_capacity(10);

    s.push('c');
    s.push('d');

    // Assertion testing
    assert_eq!(10, s.capacity());

    println!("{}", s);

}

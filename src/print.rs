pub fn run() {
    // Print to console
    println!("print.rs");

    // Can only print strings, so use formatting
    println!("{}", 1);

    // Multiple placeholders
    println!("{} is really {}", "Rust", "cool");

    // Positional args
    println!("{0} is {1} and {0} is {2}", "Rust", "cool", "fun");

    // Named args
    println!("{language} is very {speed}!", language = "Rust", speed = "fast");

    // Traits
    println!("Binary: {:b}; Hex: {:x}; Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "Brice"));

    // Math
    println!("10 + 10 = {}", 10+10)
}
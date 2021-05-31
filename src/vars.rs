// Variables are immutable by default

pub fn run() {
    let name = "Brice";
    // Add mut to make variable mutable
    // This will warn us since we don't use age when it's 17, only 18
    let mut age = 17;

    age = 18;
    
    println!("{} is {}", name, age);

    // Constants
    // Must be defined explicitly
    // Constant names should be uppercase
    const ID: i32 = 001;

    println!("{}", ID);

    // Mutiple variables
    let ( language, speed ) = ("Rust", "Fast");

    println!("{} is {}", language, speed);
}
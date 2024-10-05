// this is a single-line comment

use std::env;

fn main() {
    let my_name: &str = "Sagun";
    println!("Hello, {}!", my_name);

    let age: i32 = 30; // i32 is a 32-bit signed integer
    let temperature: f64 = 20.5; // f64 is a 64-bit floating point
    let is_active: bool = true; // Boolean type
    let initial: char = 'A'; // char represents a Unicode scalar value
    let count: u32 = 100; // u32 is a 32-bit unsigned integer
    let distance = 15.0; // Rust automatically infers distance to be f64

    println!("Age: {}", age);
    println!("Temperature: {}", temperature);
    println!("Is active: {}", is_active);
    println!("Initial: {}", initial);
    println!("Count: {}", count);
    println!("Distance: {}", distance);

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <op> <text>", args[0]);
        std::process::exit(1);
    }
    // for a in args {
    //     println!("{a}");
    // }

    let op = &args[1];
    let text = &args[2];
    println!("op: {op} text: {text}");

}
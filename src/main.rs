use std::env;
use std::fs;

// Main!
fn main() {
    // Get args from user.
    let args: Vec<_> = env::args().collect();

    // Check args length.
    if args.len() != 2 {
        println!("Expected 1 arugment. Usage: ./run.sh <filename>");
        panic!(1);
    }

    // Get filename, read raw contents.
    let filename: String = format!("{}", args[1]);
    let contents = fs::read_to_string(filename).expect("File not found");

    println!("Hello, world!");
}

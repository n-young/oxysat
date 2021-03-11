use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Expected 1 arugment. Usage: ./run.sh <filename>");
        panic!(1);
    }
    let filename: String = format!("{}", args[1]);
    let contents = fs::read_to_string(filename).expect("File not found");

    println!("Hello, world!");
}

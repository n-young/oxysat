use std::env;
use std::fs;

mod parser;
mod solver;
mod structs;

// Main!
fn main() {
    // Get args from user.
    let args: Vec<_> = env::args().collect();

    // Check args length.
    if args.len() != 2 {
        println!("Expected 1 argument. Usage: ./run.sh <filename>");
        panic!(1);
    }

    // Get filename, read raw contents.
    let filename: String = format!("{}", args[1]);
    let contents = fs::read_to_string(filename).expect("File not found");

    // Parse!
    let clauses = parser::parse(contents);
    for clause in clauses {
        println!("{}", clause);
    }

    // Solve!
    let (isSat, assignments) = solver::solve(clauses);
}

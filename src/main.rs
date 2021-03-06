use std::{env, process::exit};
use std::fs;

mod parser;
mod solver;
mod structs;
mod consts;

// Main!
fn main() {
    // Get args from user.
    let args: Vec<_> = env::args().collect();

    // Check args length.
    if args.len() != 2 {
        println!("Expected 1 argument. Usage: ./run.sh <filename>");
        exit(1);
    }

    // Get filename, read raw contents.
    let filename = format!("{}", args[1]);
    let contents = fs::read_to_string(filename).expect("File not found");

    // Parse!
    let clauses = parser::parse(contents);

    // Solve and print!
    let (is_sat, assignments) = solver::solve(clauses);
    if is_sat {
        println!("s SATISFIABLE");
        print!("v");
        for v in assignments {
            print!(" {}", v);
        }
        println!(" 0");
    } else {
        println!("s UNSATISFIABLE");
    }
}

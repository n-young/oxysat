use crate::structs::*;
use std::collections::HashSet;

// Parse raw string input into a vector of clauses.
pub fn parse(raw_input: String) -> Vec<Clause> {
    // Clean input.
    let lines = raw_input.trim().split("\n");

    // Initialize.
    let mut clause_set: Vec<Clause> = vec![];

    // Concat values into a string.
    let mut s = String::new();
    for l in lines {
        // Clean input, break on invalid, c, and p lines.
        let tokens: Vec<&str> = l.trim().split(" ").collect();
        if tokens.len() == 0 || tokens[0] == "c" || tokens[0] == "p" { continue }
        s.push_str(l);
        s.push_str(" ");
    }

    // Declare some vars.
    let mut literal_set: HashSet<Literal> = HashSet::new();
    let mut clause_id = 0;

    // Iterate through tokens.
    let tokens: Vec<&str> = s.trim().split_whitespace().collect();
    for tok in tokens {
        let tok_int = tok.parse::<i64>().expect("ERROR: Token not an integer.");
        if tok_int == 0 {
            // Make a new clause, add to clause_set.
            let clause = Clause { id: clause_id, literals: literal_set };
            clause_set.push(clause);
            clause_id += 1;
            literal_set = HashSet::new();
        } else {
            let literal = Literal::new(tok_int);
            literal_set.insert(literal);
        }
    }  

    // Return.
    clause_set
}

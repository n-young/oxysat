use crate::structs::*;

pub fn parse(raw_input: String) -> Vec<Clause> {
    // Clean input.
    let lines = raw_input.trim().split("\n");

    // Initialize.
    let mut clause_set: Vec<Clause> = vec![];

    // Iterate.
    let mut clause_id = 0;
    for l in lines {
        // Clean input, break on invalid, c, and p lines
        let tokens: Vec<&str> = l.trim().split(" ").collect();
        if tokens.len() == 0 || tokens[0] == "c" || tokens[0] == "p" { continue }

        // Iterate through tokens.
        let mut literal_set: Vec<Literal> = vec![];
        for tok in tokens {
            let tok_int = tok.parse::<i64>().expect("ERROR: Token not an integer.");
            if tok_int == 0 { break }
            let literal = Literal::new(tok_int);
            literal_set.push(literal);
        }  

        // Make a new clause, add to clause_set.
        let clause = Clause { id: clause_id, literals: literal_set };
        clause_set.push(clause);
        clause_id += 1;
    }

    // Return.
    clause_set
}

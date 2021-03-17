use crate::structs::*;
use std::collections::HashSet;

// Parse raw string input into a vector of clauses.
pub fn parse(raw_input: String) -> Vec<Clause> {
    // Clean input.
    let lines = raw_input.trim().split("\n");

    // Initialize.
    let mut clause_set = vec![];

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
    let mut literal_set = HashSet::new();
    let mut clause_id = 0;

    // Iterate through tokens.
    let tokens: Vec<&str> = s.trim().split_whitespace().collect();
    for tok in tokens {
        let tok_int = tok.parse::<i32>().expect("ERROR: Token not an integer.");
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

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_parse() {
        let result = parse(String::from(""));
        let empty_vec: Vec<Clause> = vec![];
        assert_eq!(result.len(), empty_vec.len());
    }

    #[test]
    fn ignores_c_and_p_lines() {
        let result = parse(String::from("c\np\nc\np\nc\np\nc\n"));
        let empty_vec: Vec<Clause> = vec![];
        assert_eq!(result.len(), empty_vec.len());
    }

    #[test]
    fn ignores_blank_lines() {
        let result = parse(String::from("c\n\n\n\n\n\n\n"));
        let empty_vec: Vec<Clause> = vec![];
        assert_eq!(result.len(), empty_vec.len());
    }

    #[test]
    fn one_clause() {
        let result = parse(String::from("1 2 3 0"));

        let mut clause = Clause { id: 0, literals:HashSet::new()};
        clause.literals.insert(Literal::Positive(String::from("1")));
        clause.literals.insert(Literal::Positive(String::from("2")));
        clause.literals.insert(Literal::Positive(String::from("3")));

        let clauses: Vec<Clause> = vec![clause];

        assert_eq!(result.len(), clauses.len());
        for i in 0..result.len() {
            assert_eq!(result[i], clauses[i]);
        }
    }

    #[test]
    fn two_clause() {
        let result = parse(String::from("1 2 3 0\n 1 -3 0"));

        let mut clause_1 = Clause { id: 0, literals:HashSet::new()};
        clause_1.literals.insert(Literal::Positive(String::from("1")));
        clause_1.literals.insert(Literal::Positive(String::from("2")));
        clause_1.literals.insert(Literal::Positive(String::from("3")));

        let mut clause_2 = Clause { id: 1, literals:HashSet::new()};
        clause_2.literals.insert(Literal::Positive(String::from("1")));
        clause_2.literals.insert(Literal::Negative(String::from("3")));

        let clauses: Vec<Clause> = vec![clause_1, clause_2];
        assert_eq!(result.len(), clauses.len());
        for i in 0..result.len() {
            assert_eq!(result[i], clauses[i]);
        }
    }

    #[test]
    fn clause_newlines() {
        let result = parse(String::from("1 2\n 3 0\n 1 -3\n 0"));

        let mut clause_1 = Clause { id: 0, literals:HashSet::new()};
        clause_1.literals.insert(Literal::Positive(String::from("1")));
        clause_1.literals.insert(Literal::Positive(String::from("2")));
        clause_1.literals.insert(Literal::Positive(String::from("3")));

        let mut clause_2 = Clause { id: 1, literals:HashSet::new()};
        clause_2.literals.insert(Literal::Positive(String::from("1")));
        clause_2.literals.insert(Literal::Negative(String::from("3")));

        let clauses: Vec<Clause> = vec![clause_1, clause_2];
        assert_eq!(result.len(), clauses.len());
        for i in 0..result.len() {
            assert_eq!(result[i], clauses[i]);
        }
    }
}

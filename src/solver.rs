use crate::structs::*;

pub fn solve(clauses: Vec<Clause>) -> (bool, Vec<Literal>) {
    panic!()
}

fn solve_helper(assignment: Vec<Literal>, clauses: Vec<Clause>) -> (bool, Vec<Literal>) {
    panic!()
}

// Perform unit clause elimination and return resulting clauses/assignment
fn unit_clause_elimination(
    assignment: Vec<Literal>,
    clauses: Vec<Clause>,
) -> (Vec<Literal>, Vec<Clause>) {
    panic!()
}

// Perform pure literal elimination and return resulting clauses/assignment
fn pure_literal_elimination(
    assignment: Vec<Literal>,
    clauses: Vec<Clause>,
) -> (Vec<Literal>, Vec<Clause>) {
    panic!()
}

fn pick_var(clauses: Vec<Clause>) -> (Vec<Literal>, Vec<Literal>, Vec<Clause>, Vec<Clause>) {
    // TODO: pick a random element and return the resulting formula from assigning
    // it to true/false
    panic!()
}

use crate::structs::*;
use std::iter::FilterMap;
use std::collections::HashSet;

// TODO: Make it such that no functions borrow values.

// Solve function - passes variables to the solve helper.
pub fn solve(clauses: Vec<Clause>) -> (bool, Vec<Literal>) {
    let assignment = vec![];
    solve_helper(assignment, clauses)
}

// Primary solver body. Sovles recursively.
fn solve_helper(assignment: Vec<Literal>, clauses: Vec<Clause>) -> (bool, Vec<Literal>) {
    // Apply unit clause elimination.
    let (assignment, clauses) = unit_clause_elimination(assignment, clauses);

    // Apply pure literal elimination.
    let (assignment, clauses) = pure_literal_elimination(assignment, clauses);

    // If there are no clauses, SAT.
    if clauses.len() == 0 {
        return (true, assignment);
    }

    // If there is an empty clause [], UNSAT.
    for clause in &clauses {
        if clause.literals.len() == 0 {
            return (false, vec![]);
        }
    }

    // Pick a variable and alter clauses.
    let (assignment_if_true, assignment_if_false, clauses_if_true, clauses_if_false) =
        pick_var(&clauses);

    // Evaluate if the variable we chose to be true.
    let (true_results_is_sat, assignment) = solve_helper(assignment_if_true, clauses_if_true);

    // Recurse or return.
    if true_results_is_sat {
        (true, assignment)
    } else {
        solve_helper(assignment_if_false, clauses_if_false)
    }
}

// Perform unit clause elimination and return resulting clauses/assignment
fn unit_clause_elimination(assignment: Vec<Literal>, clauses: Vec<Clause>) -> (Vec<Literal>, Vec<Clause>) { 
    // Define a boolean mask.
    let units = HashSet::new();

    // Iterate through clauses.
    for clause in &clauses {
        // If there's only one clause, continue.
        if clause.literals.len() != 1 { continue; }

        // Check if we've inserted its opposite; if so, return early.
        let literal = clause.literals[0];
        match literal {
            Literal::Positive(id) => if units.contains(Literal::Negative(id)) {
                return (vec![], vec![Clause { id: -1, literals: vec![] }])
            },
            Literal::Negative(id) => if units.contains(Literal::Positive(id)) {
                return (vec![], vec![Clause { id: -1, literals: vec![] }])
            },
        }

        // Else, mark as a unit clause.
        units.insert(literal);
    }
    
    // FilterMap (our brains are kiiiiinda big).
    clauses.filter_map(|c| )
}

// Perform pure literal elimination and return resulting clauses/assignment
fn pure_literal_elimination(assignment: Vec<Literal>, clauses: Vec<Clause>) -> (Vec<Literal>, Vec<Clause>) {
    panic!()
}

fn pick_var(clauses: &Vec<Clause>) -> (Vec<Literal>, Vec<Literal>, Vec<Clause>, Vec<Clause>) {
    // TODO: pick a random element and return the resulting formula from assigning
    // it to true/false
    panic!()
}

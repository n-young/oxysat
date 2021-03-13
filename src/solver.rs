use crate::structs::*;

pub fn solve(clauses: Vec<Clause>) -> (bool, Vec<Literal>) {
    let assignment = vec![];
    solve_helper(assignment, clauses)
}

fn solve_helper(assignment: Vec<Literal>, clauses: Vec<Clause>) -> (bool, Vec<Literal>) {
    let (assignment, clauses) = unit_clause_elimination(assignment, clauses);

    let (assignment, clauses) = pure_literal_elimination(assignment, clauses);

    if clauses.len() == 0 {
        return (true, assignment);
    }

    //  if formula has an empty clause return UNSAT
    for clause in &clauses {
        if clause.literals.len() == 0 {
            return (false, vec![]);
        }
    }

    let (assignment_if_true, assignment_if_false, clauses_if_true, clauses_if_false) =
        pick_var(clauses);

    let (true_results_is_sat, assignment) = solve_helper(assignment_if_true, clauses_if_true);

    if true_results_is_sat {
        (true, assignment)
    } else {
        solve_helper(assignment_if_false, clauses_if_false)
    }
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

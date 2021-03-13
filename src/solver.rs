use crate::structs::*;
use crate::consts;
use std::collections::HashSet;
use std::vec::Vec;
use std::iter::FromIterator;


// TODO: Make it such that no functions borrow values.

// Solve function - passes variables to the solve helper.
pub fn solve(clauses: Vec<Clause>) -> (bool, HashSet<Literal>) {
    let assignment = HashSet::new();
    solve_helper(assignment, clauses)
}

// Primary solver body. Sovles recursively.
fn solve_helper(assignment: HashSet<Literal>, clauses: Vec<Clause>) -> (bool, HashSet<Literal>) {
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
            return consts::unsat();
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
fn unit_clause_elimination(
    assignment: HashSet<Literal>,
    clauses: Vec<Clause>,
) -> (HashSet<Literal>, Vec<Clause>) {
    // Define a boolean mask.
    let mut units = HashSet::new();

    // Iterate through clauses.
    for clause in &clauses {
        // If there's only one clause, continue.
        if clause.literals.len() != 1 {
            continue;
        }

        // Check if we've inserted its opposite; if so, return early.
        let literal = Vec::from_iter(clause.literals.iter())[0];
        match literal {
            Literal::Positive(id) => {
                if units.contains(&Literal::Negative(id.clone())) {
                    return consts::unsat_ret();
                }
            }
            Literal::Negative(id) => {
                if units.contains(&Literal::Positive(id.clone())) {
                    return consts::unsat_ret();
                }
            }
        }

        // Else, mark as a unit clause.
        units.insert(literal.clone());
    }

    // FilterMap (our brains are kiiiiinda big).
    let flipped_units: HashSet<Literal> = units.iter().map(|u| u.opposite()).collect();
    let new_clauses = clauses.iter().filter_map(|c| {
        if c.literals.intersection(&units).collect::<HashSet<&Literal>>().len() > 0 {
            None
        } else {
            Some(Clause {
                id: c.id,
                literals: c.literals.difference(&flipped_units).cloned().collect::<HashSet<Literal>>(),
            })
        }
    });

    return (assignment.union(&units).cloned().collect(), new_clauses.collect());
}

// Perform pure literal elimination and return resulting clauses/assignment
fn pure_literal_elimination(
    assignment: HashSet<Literal>,
    clauses: Vec<Clause>,
) -> (HashSet<Literal>, Vec<Clause>) {
    panic!()
}

fn pick_var(clauses: &Vec<Clause>) -> (HashSet<Literal>, HashSet<Literal>, Vec<Clause>, Vec<Clause>) {
    // TODO: pick a random element and return the resulting formula from assigning
    // it to true/false
    panic!()
}

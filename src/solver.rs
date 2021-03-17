use crate::structs::*;
use crate::consts;
use std::{collections::{HashSet, HashMap}};
use std::vec::Vec;
use std::iter::FromIterator;


// Solve function - passes variables to the solve helper.
pub fn solve(clauses: Vec<Clause>) -> (bool, HashSet<Literal>) {    
    // Get all of the positive literals.
    let mut all_literals = HashSet::new();
    for clause in &clauses {
        for literal in &clause.literals {
            match literal {
                Literal::Positive(_) => all_literals.insert(literal.clone()),
                Literal::Negative(_) => all_literals.insert(literal.opposite()),
            };
        }
    }

    // Run.
    let assignment = HashSet::new();
    let (is_sat, mut assignment) = solve_helper(assignment, clauses);

    // Return.
    if !is_sat {
        (is_sat, assignment)
    } else {
        for lit in &all_literals {
            if !assignment.contains(&lit.opposite()) {
                assignment.insert(lit.clone());
            }
        }
        (is_sat, assignment)
    }
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
        pick_var(assignment, &clauses);

    // Evaluate if the variable we chose to be true.
    let (true_results_in_sat, assignment) = solve_helper(assignment_if_true, clauses_if_true);

    // Recurse or return.
    if true_results_in_sat {
        (true, assignment)
    } else {
        solve_helper(assignment_if_false, clauses_if_false)
    }
}


// Perform unit clause elimination and return resulting clauses/assignment
fn unit_clause_elimination(assignment: HashSet<Literal>, clauses: Vec<Clause>)
        -> (HashSet<Literal>, Vec<Clause>) {
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
                literals: c.literals.difference(&flipped_units).cloned().collect(),
            })
        }
    });

    // Return!
    (assignment.union(&units).cloned().collect(), new_clauses.collect())
}


// Perform pure literal elimination and return resulting clauses/assignment
fn pure_literal_elimination(assignment: HashSet<Literal>, clauses: Vec<Clause>)
        -> (HashSet<Literal>, Vec<Clause>) {
    // Define a boolean mask.
    let mut pure_literals: HashMap<Literal, bool> = HashMap::new();

    // Iterate through the clauses and literals.
    for clause in &clauses {
        for literal in clause.literals.iter() {
            match pure_literals.get(&literal.opposite()) {
                None => {pure_literals.insert(literal.clone(), true);},
                Some(_) => {
                    pure_literals.insert(literal.clone(), false);
                    pure_literals.insert(literal.opposite(), false);
                },
            }
        }
    }

    // Make a set of pure literals.
    let pure_literals = pure_literals.keys().filter(|k| *pure_literals.get(k).unwrap_or(&false)).cloned().collect();

    // Filter out clauses that have a pure literal.
    let clauses = clauses.iter().filter(|c| c.literals.intersection(&pure_literals).collect::<Vec<&Literal>>().len() == 0);

    // Return!
    (assignment.union(&pure_literals).cloned().collect(), clauses.cloned().collect::<Vec<Clause>>())
}


// Picks a variable, alters the clauses appropriately.
fn pick_var(assignment: HashSet<Literal>, clauses: &Vec<Clause>) -> (HashSet<Literal>, HashSet<Literal>, Vec<Clause>, Vec<Clause>) {
    // Pick a variable, put it and its opposite into sets.

    let mut literal_count = HashMap::new();

    for clause in clauses {
        for literal in &(clause.literals) {
            literal_count.insert(literal, literal_count.get(literal).unwrap_or(&0) + 1);
        }
    }
    let most_common_var = literal_count.iter().max_by(|(_,x1), (_,x2)| x1.cmp(x2)).unwrap().0;

    // let picked_var = clauses.get(0).unwrap().literals.iter().cloned().collect::<Vec<Literal>>().get(0).expect("Shouldn't be an empty clause").clone();
    let picked_var = (*most_common_var).clone();
    let picked_var_set = HashSet::from_iter(vec![picked_var.clone()]);
    let picked_var_opposite_set = HashSet::from_iter(vec![picked_var.opposite()]);

    // Get the clauses if the picked variable is true.
    let clauses_if_true = clauses.iter().filter_map(|c| {
        if c.literals.contains(&picked_var) {
            None
        } else {
            let clause_with_var_removed = c.literals.difference(&picked_var_opposite_set).cloned().collect::<HashSet<Literal>>();
            Some(Clause {
                id: c.id,
                literals: clause_with_var_removed,
            })
        }
    });

    // Get the clauses if the picked variable is false.
    let clauses_if_false = clauses.iter().filter_map(|c| {
        if c.literals.contains(&picked_var.opposite()) {
            None
        } else {
            let clause_with_var_removed = c.literals.difference(&picked_var_set).cloned().collect::<HashSet<Literal>>();
            Some(Clause {
                id: c.id,
                literals: clause_with_var_removed,
            })
        }
    });

    // Return!
    (assignment.union(&picked_var_set).cloned().collect(), assignment.union(&picked_var_opposite_set).cloned().collect(), clauses_if_true.collect(), clauses_if_false.collect())
}

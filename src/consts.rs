use std::collections::HashSet;
use crate::structs::*;

// The value to return to `main` for an unsatisfiable formula.
pub fn unsat() -> (bool, HashSet<Literal>) {
    (false, HashSet::new())
}

// The value to return to the solver for an unsatisfiable formula.
pub fn unsat_ret() -> (HashSet<Literal>, Vec<Clause>) {
    (HashSet::new(),
    vec![Clause {
        id: -1,
        literals: HashSet::new(),
    }])
}

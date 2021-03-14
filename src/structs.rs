use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

// Literal type - is either positive or negative, has a String id.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Literal {
    Positive(String),
    Negative(String),
}
impl Literal {
    // Constructor.
    pub fn new(i: i32) -> Literal {
        if i > 0 {
            Literal::Positive(i.to_string())
        } else if i < 0 {
            Literal::Negative((-i).to_string())
        } else {
            panic!("ERROR: 0 shouldn't be a literal.")
        }
    }

    // Opposite.
    pub fn opposite(&self) -> Literal {
        match self {
            Literal::Positive(id) => Literal::Negative(id.clone()),
            Literal::Negative(id) => Literal::Positive(id.clone()),
        }
    }
}
impl Display for Literal {
    // Formatter for a Literal.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Positive(id) => write!(f, "{}", id),
            Literal::Negative(id) => write!(f, "-{}", id),
        }
    }
}


// Clause struct - has an ID and a set of Literals.
#[derive(Debug, Clone)]
pub struct Clause {
    pub id: i32,
    pub literals: HashSet<Literal>,
}
impl Display for Clause {
    // Formatter for a Clause.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_strings: Vec<_> = self.literals.iter().map(|x| x.to_string()).collect();
        write!(
            f,
            "ID: {} Literals: {}",
            self.id,
            literal_strings.join(", ")
        )
    }
}

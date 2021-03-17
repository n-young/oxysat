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
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Clause {
    pub id: i32,
    pub literals: HashSet<Literal>,
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_construction() {
        let lit = Literal::Positive(String::from("name"));
        match lit {
            Literal::Positive(id) => assert_eq!(id, "name"),
            _ => {}
        }

        let lit = Literal::Negative(String::from("name"));
        match lit {
            Literal::Negative(id) => assert_eq!(id, "name"),
            _ => {}
        }
    }

    #[test]
    fn test_opposite() {
        let pos = Literal::Positive(String::from("id"));
        let neg = Literal::Negative(String::from("id"));
        assert_eq!(pos, neg.opposite());
        assert_eq!(pos.opposite(), neg);
    }

    #[test]
    fn test_literal_display() {
        let pos = Literal::Positive(String::from("1"));
        let neg = Literal::Negative(String::from("3"));
        assert_eq!(String::from("1"), format!("{}", pos));
        assert_eq!(String::from("-3"), format!("{}", neg));
    }
}
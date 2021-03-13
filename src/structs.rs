use std::fmt;
use std::fmt::Display;

// Literal type - is either positive or negative, has a String id.
#[derive(Hash, Eq, Debug)]
pub enum Literal {
    Positive(String),
    Negative(String),
}

impl Literal {
    // Constructor.
    pub fn new(i: i64) -> Literal {
        if i > 0 {
            Literal::Positive(i.to_string())
        } else if i < 0 {
            Literal::Negative((-i).to_string())
        } else {
            panic!("ERROR: 0 shouldn't be a literal.")
        }
    }

    // Opposite.
    pub fn opposite(l: Literal) -> Literal {
        match l {
            Literal::Positive(id) => Literal::Negative(id),
            Literal::Negative(id) => Literal::Positive(id),
        }
    }

    // ID Getter.
    pub fn id(&self) -> &String {
        match self {
            Literal::Positive(id) => id,
            Literal::Negative(id) => id,
        }
    }
}

// Formatter for a Literal.
impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Positive(id) => write!(f, "{}", id),
            Literal::Negative(id) => write!(f, "-{}", id),
        }
    }
}

// Clause struct - has an ID and a set of Literals.
#[derive(Debug)]
pub struct Clause {
    pub id: i64,
    pub literals: Vec<Literal>,
}

// Formatter for a Clause.
impl Display for Clause {
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

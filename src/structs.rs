use std::fmt;
use std::fmt::Display;
// Sign enum - either Positive or Negative.
pub enum Sign {
    Positive,
    Negative,
}

// Literal struct - has a name and a Sign.
pub struct Literal {
    pub name: String,
    pub sign: Sign,
}

// constructor
impl Literal {
    pub fn new(i: i64) -> Literal {
        if i < 0 {
            Literal { name: (-i).to_string(), sign: Sign::Negative }
        } else if i > 0 {
            Literal { name: i.to_string(), sign: Sign::Positive }
        } else {
            panic!("ERROR: 0 shouldn't be a literal.")
        }
    }
}

// Formatter for a Literal.
impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sign {
            Sign::Positive => write!(f, "{}", self.name),
            Sign::Negative => write!(f, "-{}", self.name),
        }
    }
}

// Clause struct - has an ID and a set of Literals.
pub struct Clause {
    pub id: i64,
    pub literals: Vec<Literal>,
}

// Formatter for a Clause.
impl Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_strings: Vec<_> = self.literals.iter().map(|x| x.to_string()).collect();
        write!(f, "ID: {} Literals: {}", self.id, literal_strings.join(", "))
    }
}

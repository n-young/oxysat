use std::fmt;
use std::fmt::Display;
// Sign enum - either Positive or Negative.
pub enum Sign {
    Position,
    Negative,
}

// Literal struct - has a name and a Sign.
pub struct Literal {
    pub name: String,
    pub sign: Sign,
}

// Formatter for a Literal.
impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sign {
            Positive => write!(f, "{}", self.name),
            Negative => write!(f, "-{}", self.name),
        }
    }
}

// Clause struct - has an ID and a set of Literals.
pub struct Clause {
    pub id: i64,
    pub literal: Vec<Literal>,
}

// Formatter for a Clause.
impl Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {} Literals: {}", self.id, self.literal.join(", "))
    }
}

use std::fmt;
pub enum Sign {
    Position,
    Negative,
}

pub struct Literal {
    pub name: String,
    pub sign: Sign,
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sign {
            Positive => write!(f, "{}", self.name),
            Negative => write!(f, "-{}", self.name),
        }
    }
}

pub struct Clause {
    pub id: i64,
    pub literal: Vec<Literal>,
}

impl Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {} Literals: {}", self.id, self.literal.join(", "))
    }
}

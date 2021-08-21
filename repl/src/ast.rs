use std::fmt;

#[derive(PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => write!(f, "{}", value),
            Self::Num(value) => write!(f, "{}", value),
            Self::Str(value) => write!(f, "{}", value),
            Self::Null => write!(f, "null"),
        }
    }
}

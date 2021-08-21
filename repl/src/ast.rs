use crate::lex::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    // Declarations
    Var { name: String, value: Option<Expr> },

    // Not Declaration
    Comment(String),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Unary {
        op: Token,
        expr: Box<Expr>,
    },
    Literal(Value),
    Grouping(Box<Expr>),
}

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

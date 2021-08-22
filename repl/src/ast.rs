use crate::lex::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    // Declarations
    VariableDeclaration {
        name: String,
        value: Option<Expr>,
    },
    ConstDeclaration {
        name: String,
        value: Expr,
    },
    FnDeclaration {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },

    // Not Declaration
    If {
        condition: Expr,
        then: Box<Stmt>,
        otherwise: Option<Box<Stmt>>,
    },
    Loop {
        label: Option<String>,
        body: Box<Stmt>,
    },
    Block(Vec<Stmt>),
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

    Variable(String),
    Assignment(String, Box<Expr>),
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

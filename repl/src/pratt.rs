use crate::lex::Token;

// Pratt parser rules based on the one from the book "Crafting Interpreters"

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

#[derive(Debug, PartialEq)]
pub enum ParseFn {
    None,
    Unary,
    Binary,
    Grouping,
    Number,
}

pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}

pub fn get_rule(operator: &Token) -> ParseRule {
    match operator {
        Token::LeftParen => ParseRule {
            prefix: ParseFn::Grouping,
            infix: ParseFn::None,
            precedence: Precedence::None,
        },
        Token::Minus => ParseRule {
            prefix: ParseFn::Unary,
            infix: ParseFn::Binary,
            precedence: Precedence::Term,
        },
        Token::Plus => ParseRule {
            prefix: ParseFn::None,
            infix: ParseFn::Binary,
            precedence: Precedence::Term,
        },
        Token::Slash => ParseRule {
            prefix: ParseFn::None,
            infix: ParseFn::Binary,
            precedence: Precedence::Factor,
        },
        Token::Star => ParseRule {
            prefix: ParseFn::None,
            infix: ParseFn::Binary,
            precedence: Precedence::Factor,
        },
        Token::Num => ParseRule {
            prefix: ParseFn::Number,
            infix: ParseFn::None,
            precedence: Precedence::None,
        },
        _ => ParseRule {
            prefix: ParseFn::None,
            infix: ParseFn::None,
            precedence: Precedence::None,
        },
    }
}

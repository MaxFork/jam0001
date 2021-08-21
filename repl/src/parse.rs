use crate::ast::Value;
use crate::lex::{Lexer, Token};
use crate::pratt::{get_rule, ParseFn, Precedence};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Unary {
        op: Token,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Literal(Value),
}

pub struct Parser<'source> {
    lexer: Lexer<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Lexer::new(source),
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.parse_precedence(Precedence::Assignment)
    }

    fn binary(&mut self, left: Box<Expr>) -> Expr {
        let op = self.lexer.next().unwrap();
        let right = Box::new(self.parse_precedence(Precedence::Unary));
        Expr::Binary { left, op, right }
    }

    fn unary(&mut self) -> Expr {
        let op = self.lexer.next().unwrap();
        let expr = Box::new(self.parse_precedence(Precedence::Unary));
        Expr::Unary { op, expr }
    }

    fn number(&mut self) -> Expr {
        self.lexer.next().unwrap();
        Expr::Literal(Value::Num(self.lexer.slice().parse().unwrap()))
    }

    fn parse_precedence(&mut self, prec: Precedence) -> Expr {
        let peek = self.lexer.peek().unwrap();
        let prefix_rule = get_rule(peek).prefix;
        let mut left = self.parse_by_rule(prefix_rule, None);

        while self.lexer.peek().is_some() && prec <= get_rule(self.lexer.peek().unwrap()).precedence
        {
            let infix_rule = get_rule(self.lexer.peek().unwrap()).infix;
            left = self.parse_by_rule(infix_rule, Some(Box::new(left)));
        }

        left
    }

    fn parse_by_rule(&mut self, rule: ParseFn, operand: Option<Box<Expr>>) -> Expr {
        match rule {
            ParseFn::Unary => self.unary(),
            ParseFn::Binary => self.binary(operand.unwrap()),
            ParseFn::Number => self.number(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unary() {
        let program = "-1";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse(),
            Expr::Unary {
                op: Token::Minus,
                expr: Box::new(Expr::Literal(Value::Num(1f64)))
            }
        );
    }

    #[test]
    fn binary() {
        let program = "1+2";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse(),
            Expr::Binary {
                left: Box::new(Expr::Literal(Value::Num(1f64))),
                op: Token::Plus,
                right: Box::new(Expr::Literal(Value::Num(2f64)))
            }
        );
    }
}

use crate::ast::{Expr, Stmt, Value};
use crate::lex::{Lexer, Token};
use crate::pratt::{get_rule, ParseFn, Precedence};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unexpected token")]
    UnexpectedToken(Token),

    #[error("expected token")]
    ExpectedToken(Vec<Token>, Token), // expected, got

    #[error("expected an expression")]
    ExpectedExpression,

    #[error("type coercion error")]
    TypeCoercion(#[from] std::num::ParseFloatError),
}

pub type ParserResult = Result<Stmt, ParserError>;

pub struct Parser<'source> {
    lexer: Lexer<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Lexer::new(source),
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        self.declaration()
    }

    fn declaration(&mut self) -> ParserResult {
        if self.par(&[Token::Let]) {
            return self.variable_declaration();
        }

        self.statement()
    }

    fn variable_declaration(&mut self) -> ParserResult {
        let _keyword = self.lexer.next(); // will use this later for error prompts
        self.must_be_next(&[Token::Ident])?;
        let name = self.lexer.slice().to_string();

        let mut value: Option<Expr> = None;

        if self.par(&[Token::Equal]) {
            self.lexer.next();
            value = Some(self.expression()?);
        }

        Ok(Stmt::VariableDeclaration { name, value })
    }

    fn statement(&mut self) -> ParserResult {
        if self.par(&[Token::Comment]) {
            let comment = &self.lexer.slice()[1..].trim();
            return Ok(Stmt::Comment(comment.to_string()));
        }

        Ok(Stmt::Expr(self.expression()?))
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn binary(&mut self, left: Box<Expr>) -> Result<Expr, ParserError> {
        let op = self.must_be_next(&[
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::EqualEqual,
            Token::BangEqual,
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ])?;
        let precedence = get_rule(&op).get_next_precedence();
        let right = Box::new(self.parse_precedence(precedence)?);
        Ok(Expr::Binary { left, op, right })
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        let op = self.must_be_next(&[Token::Bang, Token::Minus])?;
        let expr = Box::new(self.parse_precedence(Precedence::Unary)?);
        Ok(Expr::Unary { op, expr })
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        let value = self.must_be_next(&[
            Token::True,
            Token::False,
            Token::Num,
            Token::Str,
            Token::Null,
        ])?;

        let value = match value {
            Token::True => Value::Bool(true),
            Token::False => Value::Bool(false),
            Token::Num => Value::Num(self.lexer.slice().parse()?),
            Token::Str => {
                let slice = self.lexer.slice();
                Value::Str(slice[1..slice.len() - 1].into())
            }
            Token::Null => Value::Null,
            _ => unreachable!(),
        };

        Ok(Expr::Literal(value))
    }

    fn grouping(&mut self) -> Result<Expr, ParserError> {
        let _open_paren = self.must_be_next(&[Token::LeftParen])?; // will use this later for errors
        let value = Expr::Grouping(Box::new(self.expression()?));
        self.must_be_next(&[Token::RightParen])?;
        Ok(value)
    }

    fn variable(&mut self) -> Result<Expr, ParserError> {
        let name = self.lexer.slice().to_string();

        self.lexer.next();
        if self.par(&[Token::Equal]) {
            self.lexer.next();
            let value = self.expression()?;
            return Ok(Expr::Assignment(name, Box::new(value)));
        }
        Ok(Expr::Variable(name))
    }

    fn parse_precedence(&mut self, prec: Precedence) -> Result<Expr, ParserError> {
        let peek = self.lexer.peek().unwrap();
        let prefix_rule = get_rule(peek).prefix;
        let mut left = self.parse_by_rule(prefix_rule, None)?;

        while self.lexer.peek().is_some() && prec <= get_rule(self.lexer.peek().unwrap()).precedence
        {
            let infix_rule = get_rule(self.lexer.peek().unwrap()).infix;
            left = self.parse_by_rule(infix_rule, Some(Box::new(left)))?;
        }

        Ok(left)
    }

    fn parse_by_rule(
        &mut self,
        rule: ParseFn,
        operand: Option<Box<Expr>>,
    ) -> Result<Expr, ParserError> {
        match rule {
            ParseFn::Unary => self.unary(),
            ParseFn::Binary => self.binary(operand.ok_or(ParserError::ExpectedExpression)?),
            ParseFn::Grouping => self.grouping(),
            ParseFn::Literal => self.primary(),
            ParseFn::Variable => self.variable(),
            ParseFn::None => unreachable!(),
        }
    }
}

// Helpers
impl<'source> Parser<'source> {
    // why the name par if you asked
    // it was meant to be called `match` but
    // since `match` is a reserved keyword in rust
    // I google-translated `match` to latin and
    // it says `par` so here it is
    fn par(&mut self, tokens: &[Token]) -> bool {
        if let Some(token) = self.lexer.peek() {
            return tokens.iter().any(|k| k == token);
        }
        false
    }

    fn must_be_next(&mut self, tokens: &[Token]) -> Result<Token, ParserError> {
        if let Some(token) = self.lexer.next() {
            if !tokens.iter().any(|k| *k == token) {
                return Err(ParserError::ExpectedToken(tokens.to_vec(), token));
            }
            return Ok(token);
        } else {
            return Err(ParserError::ExpectedExpression);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments() -> Result<(), Box<dyn std::error::Error>> {
        let program = "# > first class :)";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Comment("> first class :)".to_string())
        );
        Ok(())
    }

    #[test]
    fn literal() -> Result<(), Box<dyn std::error::Error>> {
        let program = "1";
        let mut parser = Parser::new(program);

        assert_eq!(parser.parse()?, Stmt::Expr(Expr::Literal(Value::Num(1f64))));
        Ok(())
    }

    #[test]
    fn unary() -> Result<(), Box<dyn std::error::Error>> {
        let program = "-1";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Expr(Expr::Unary {
                op: Token::Minus,
                expr: Box::new(Expr::Literal(Value::Num(1f64)))
            })
        );
        Ok(())
    }

    #[test]
    fn binary() -> Result<(), Box<dyn std::error::Error>> {
        let program = "1+2";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Literal(Value::Num(1f64))),
                op: Token::Plus,
                right: Box::new(Expr::Literal(Value::Num(2f64)))
            })
        );
        Ok(())
    }

    #[test]
    fn grouping() -> Result<(), Box<dyn std::error::Error>> {
        let program = "(1)";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Expr(Expr::Grouping(Box::new(Expr::Literal(Value::Num(1f64)))))
        );
        Ok(())
    }

    #[test]
    fn precedence() -> Result<(), Box<dyn std::error::Error>> {
        let program = "1+2*3-4";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal(Value::Num(1f64))),
                    op: Token::Plus,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Literal(Value::Num(2f64))),
                        op: Token::Star,
                        right: Box::new(Expr::Literal(Value::Num(3f64))),
                    })
                }),
                op: Token::Minus,
                right: Box::new(Expr::Literal(Value::Num(4f64))),
            })
        );
        Ok(())
    }

    #[test]
    fn strings() -> Result<(), Box<dyn std::error::Error>> {
        let program = r#" "foo" + "bar" "#;
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Expr(Expr::Binary {
                left: Box::new(Expr::Literal(Value::Str("foo".to_string()))),
                op: Token::Plus,
                right: Box::new(Expr::Literal(Value::Str("bar".to_string())))
            })
        );
        Ok(())
    }

    #[test]
    fn variable_expression() -> Result<(), Box<dyn std::error::Error>> {
        let program = "foo";
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Expr(Expr::Variable("foo".to_string()))
        );
        Ok(())
    }

    #[test]
    fn assignment_expression() -> Result<(), Box<dyn std::error::Error>> {
        let program = r#" foo = "bar" "#;
        let mut parser = Parser::new(program);

        assert_eq!(
            parser.parse()?,
            Stmt::Expr(Expr::Assignment(
                "foo".to_string(),
                Box::new(Expr::Literal(Value::Str("bar".to_string())))
            ))
        );
        Ok(())
    }
}

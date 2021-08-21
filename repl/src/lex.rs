use logos::{Lexer as LogosLexer, Logos};
use std::fmt;

pub struct Lexer<'source> {
    lexer: LogosLexer<'source, Token>,
    peeked: Option<Option<Token>>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        let lexer = &mut self.lexer;
        self.peeked.get_or_insert_with(|| lexer.next()).as_ref()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.lexer.next()
        }
    }
}

fn lex_str(lex: &mut LogosLexer<Token>) -> Option<String> {
    Some(lex.slice().parse::<String>().ok()?)
}

#[rustfmt::skip]
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
	// Single-character tokens
	#[token("(")] LeftParen,
	#[token(")")] RightParen,
	#[token("{")] LeftBrace,
	#[token("}")] RightBrace,
	#[token(",")] Comma,
	#[token(";")] SemiColon,
	#[token(".")] Dot,

    // Operators
	#[token("-")] Minus,
	#[token("+")] Plus,
	#[token("/")] Slash,
	#[token("*")] Star,

	// Comparison
	#[token("!")] Bang,
	#[token("!=")] BangEqual,
	#[token("=")] Equal,
	#[token("==")] EqualEqual,
	#[token(">")] Greater,
	#[token(">=")] GreaterEqual,
	#[token("<")] Less,
	#[token("<=")] LessEqual,

    // Logical
	#[token("&&")] And,
	#[token("||")] Or,

    // Keywords
	#[token("let")] Let,
	#[token("const")] Const,
	#[token("fn")] Func,

	#[token("null")] Null,
	#[token("true")] True,
	#[token("false")] False,

	#[token("loop")] Loop,
	#[token("break")] Break,
	#[token("continue")] Continue,
	#[token("return")] Return,

	#[token("if")] If,
	#[token("else")] Else,

    // Literals
    #[regex(r"#[^\n\r]*", lex_str)]
    Comment(String),

    #[regex(r"[_A-z]\w*", lex_str)]
    Ident(String),

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, lex_str)]
    Str(String),

    #[regex(r"\d+(\.*\d+)?", lex_str)]
    Num(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let program = r"
            # comment
            let foo = 1 + 2
            ";
        let tokens = Lexer::new(program).collect::<Vec<_>>();

        assert_eq!(
            tokens,
            vec![
                Token::Comment("# comment".to_string()),
                Token::Let,
                Token::Ident("foo".to_string()),
                Token::Equal,
                Token::Num("1".to_string()),
                Token::Plus,
                Token::Num("2".to_string()),
            ]
        );
    }
}
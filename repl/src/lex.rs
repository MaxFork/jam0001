use logos::{Lexer as LogosLexer, Logos, Span};
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

    pub fn span(&mut self) -> Span {
        self.lexer.span()
    }

    pub fn slice(&mut self) -> &str {
        self.lexer.slice()
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
    #[regex(r"#[^\n\r]*")]
    Comment,

    #[regex(r"[_A-z]\w*")]
    Ident,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    Str,

    #[regex(r"\d+(\.*\d+)?")]
    Num,

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
        let mut lex = Lexer::new(program);

        assert_eq!(lex.next(), Some(Token::Comment));
        assert_eq!(lex.slice(), "# comment");

        assert_eq!(lex.next(), Some(Token::Let));

        assert_eq!(lex.next(), Some(Token::Ident));
        assert_eq!(lex.slice(), "foo");

        assert_eq!(lex.next(), Some(Token::Equal));

        assert_eq!(lex.next(), Some(Token::Num));
        assert_eq!(lex.slice(), "1");

        assert_eq!(lex.next(), Some(Token::Plus));

        assert_eq!(lex.next(), Some(Token::Num));
        assert_eq!(lex.slice(), "2");

        assert_eq!(lex.next(), None);
    }
}

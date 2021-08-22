#![deny(unsafe_code)]
#![feature(format_args_capture)]
#![feature(decl_macro)]

pub mod ast;
pub mod interpret;
pub mod lex;
pub mod parse;
pub mod pratt;

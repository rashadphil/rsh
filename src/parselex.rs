use chumsky::Parser;

pub mod lex;
pub mod parser;

pub type Span = std::ops::Range<usize>;

pub fn lex(input: &str) -> Vec<(lex::Token, Span)> {
    lex::lexer().parse(input).unwrap_or_default() 
}

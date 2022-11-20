use core::fmt;

use chumsky::{prelude::*, Stream};
use chumsky::{
    primitive::{filter, just},
    text::TextParser,
    Parser,
};

use derive_new::new;

use super::lex::{lexer, Token};
use super::Span;

#[derive(Debug, Clone)]
pub enum Val {
    Bool(bool),
    String(String),
    List(Vec<Val>),
    Num(i64),
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Bool(_) => todo!(),
            Val::String(s) => write!(f, "{}", s),
            Val::List(_) => todo!(),
            Val::Num(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Val(Val),
    LambdaExpr(Val, Box<Expr>),
    Command(Val, Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Val(v) => write!(f, "{}", v),
            Expr::LambdaExpr(_, _) => todo!(),
            Expr::Command(_, _) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<Expr>,
}

impl ParsedCommand {
    fn from_expr(expression: Expr) -> Self {
        match expression {
            Expr::Command(name, args) => match name {
                Val::String(name) => ParsedCommand { name, args },
                _ => panic!("Failed to parse!"),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, new)]
pub struct ParsedPipeline {
    pub commands: Vec<ParsedCommand>,
}

// Does not accept whitespace tokens!!!
fn ast_builder() -> impl Parser<Token, ParsedPipeline, Error = Simple<Token>> {
    let num_ident = filter_map(|span, tok: Token| match tok {
        Token::Item(item) => Ok(Val::String(item)),
        Token::Num(n) => Ok(Val::Num(n)),
        Token::QuotedItem(item) => Ok(Val::String(item)),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let args = num_ident.repeated();

    // name of command followed by arguments
    let command = num_ident.then(args).map(|(name, command_args)| {
        let args_expr = command_args.into_iter().map(Expr::Val).collect();
        let command_expr = Expr::Command(name, args_expr);
        ParsedCommand::from_expr(command_expr)
    });

    // commands seperated by a Pipe
    command
        .separated_by(just(Token::Pipe))
        .map(ParsedPipeline::new)
}

pub fn parse(query: impl Into<String>) -> ParsedPipeline {
    let query: String = query.into();
    let len = query.chars().count();

    let (tokens, mut err) = lexer().parse_recovery(query);

    // TODO : Error Handling
    let clean_tokens: Vec<(Token, Span)> = match tokens {
        Some(toks) => toks
            .into_iter()
            .filter(|(tok, _)| !matches!(tok, Token::Whitespace))
            .collect(),
        None => vec![],
    };

    let (ast, parse_errors) =
        ast_builder().parse_recovery(Stream::from_iter(len..len + 1, clean_tokens.into_iter()));

    ast.unwrap()
}

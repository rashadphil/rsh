use chumsky::{prelude::*, Stream};
use chumsky::{
    primitive::{filter, just},
    text::{TextParser},
    Parser,
};

use derive_new::new;

pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Token {
    Item(String),
    Pipe,
    Arrow,
    Dot,
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let is_word_char = |c: &char| {
        c.is_ascii_alphabetic() || c.is_ascii_alphanumeric() || c.clone() == '_' || c.clone() == '/'
    };

    let item = filter::<_, _, Simple<char>>(move |c: &char| is_word_char(c))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Item);

    let quoted_item = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Item);

    let pipe = just("|").to(Token::Pipe);
    let arrow = just("->").to(Token::Arrow);
    let dot = just(".").to(Token::Dot);

    let token = item
        .or(quoted_item)
        .or(pipe)
        .or(arrow)
        .or(dot)
        .recover_with(skip_then_retry_until([]));

    token
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}

#[derive(Debug, Clone)]
pub enum Val {
    Bool(bool),
    String(String),
    List(Vec<Val>),
    Num(f64),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Val(Val),
    LambdaExpr(Val, Box<Expr>),
    Command(Val, Vec<Box<Expr>>),
}

#[derive(Debug)]
pub struct ParsedCommand {
    name: String,
    args: Vec<Box<Expr>>,
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
    commands: Vec<ParsedCommand>,
}

fn ast_builder() -> impl Parser<Token, ParsedPipeline, Error = Simple<Token>> {
    let ident = filter_map(|span, tok: Token| match tok {
        Token::Item(item) => Ok(Val::String(item.clone())),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
    });

    let args = ident.clone().repeated();

    // name of command followed by arguments
    let command = ident.then(args).map(|(name, command_args)| {
        let args_expr = command_args
            .into_iter()
            .map(|arg| Box::new(Expr::Val(arg)))
            .collect();
        let command_expr = Expr::Command(name, args_expr);
        ParsedCommand::from_expr(command_expr)
    });

    // commands seperated by a Pipe
    let pipeline = command
        .separated_by(just(Token::Pipe))
        .map(|commands| ParsedPipeline::new(commands));

    pipeline
}

pub fn parse(query: impl Into<String>) -> ParsedPipeline {
    let query: String = query.into();
    let len = query.chars().count();

    let (tokens, mut err) = lexer().parse_recovery(query);

    // TODO : Error Handling
    let tokens = tokens.unwrap();

    let (ast, parse_errors) =
        ast_builder().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

    ast.unwrap()
}

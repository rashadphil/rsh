use chumsky::prelude::*;
use chumsky::{
    primitive::{filter, just},
    Parser,
};

use super::Span;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token {
    Num(i64),
    Item(String),
    OpenQuote,
    QuotedItem(String),
    Pipe,
    Arrow,
    Dot,
    Whitespace,
}

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let number = text::int::<_, Simple<char>>(10).map(|s| Token::Num(s.parse().unwrap()));

    let is_word_char = |c: &char| *c != ' ';

    let item = filter::<_, _, Simple<char>>(move |c: &char| is_word_char(c))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Item);

    let quoted_item = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::QuotedItem);

    let open_quote = just('"').to(Token::OpenQuote);

    let pipe = just("|").to(Token::Pipe);
    let arrow = just("->").to(Token::Arrow);
    let dot = just(".").to(Token::Dot);

    let whitespace = filter::<_, _, Simple<char>>(move |c: &char| c.is_whitespace())
        .repeated()
        .at_least(1)
        .to(Token::Whitespace);

    let token = number
        .or(quoted_item)
        .or(open_quote)
        .or(pipe)
        .or(arrow)
        .or(dot)
        .or(whitespace)
        .or(item)
        .recover_with(skip_then_retry_until([]));

    token.map_with_span(|tok, span| (tok, span)).repeated()
}

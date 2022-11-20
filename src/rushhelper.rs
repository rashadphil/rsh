use rustyline::hint::HistoryHinter;
use rustyline::CompletionType;
use rustyline::{completion::FilenameCompleter, highlight::Highlighter};
use std::borrow::Cow::{Borrowed, Owned};
use std::fs::{File, OpenOptions};
use std::io::Write;

use colored::*;
use rustyline_derive::{Completer, Helper, Hinter, Validator};

use crate::parselex::lex::Token;
use crate::parselex::{self, Span};

#[derive(Helper, Completer, Hinter, Validator)]
pub struct RushHelper {
    #[rustyline(Completer)]
    pub completer: FilenameCompleter,
    #[rustyline(Hinter)]
    pub hinter: HistoryHinter,
}

#[derive(Debug, Clone, Copy)]
enum LexState {
    Command,
    Arg,
    Quoting,
}

impl Highlighter for RushHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        let colored = hint.truecolor(140, 140, 140);
        Owned(format!("{}", colored))
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        let lexed = parselex::lex(line);

        let mut highlighted = String::new();

        let mut state = LexState::Command;

        for (token, span) in lexed {
            let slice = &line[span];

            let (colored_slice, new_state) = match token {
                Token::Num(_) => (slice.blue(), state),
                Token::Item(_) => match state {
                    LexState::Command => (slice.white().bold(), LexState::Arg),
                    LexState::Arg => (slice.cyan(), state),
                    LexState::Quoting => (slice.red(), state),
                },
                Token::Pipe => (slice.blue().bold(), LexState::Command),
                Token::Arrow => (slice.red().bold(), state),
                Token::Dot => (slice.magenta(), state),
                Token::Whitespace => (slice.normal(), state),
                Token::QuotedItem(_) => (slice.bright_green(), state),
                Token::OpenQuote => (slice.red(), LexState::Quoting),
                Token::Equal => (slice.blue().bold(), state),
            };
            state = new_state;
            highlighted.push_str(&colored_slice.to_string());
        }

        Owned(highlighted)
    }

    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str,
        completion: CompletionType,
    ) -> std::borrow::Cow<'c, str> {
        let colored = candidate.magenta();
        Owned(format!("{}", colored))
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        true
    }
}

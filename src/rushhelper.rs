use rustyline::completion::{Completer, Pair};
use rustyline::hint::HistoryHinter;
use rustyline::CompletionType;
use rustyline::{completion::FilenameCompleter, highlight::Highlighter};
use std::borrow::Cow::Owned;
use std::path::PathBuf;
use std::{env, fs};

use colored::*;
use rustyline_derive::{Completer, Helper, Hinter, Validator};

use crate::context::Context;
use crate::error::ShellError;
use crate::parselex::lex::Token;
use crate::parselex::{self};

#[derive(Default)]
pub struct RushCompleter {
    pub file_completer: FilenameCompleter,
}

impl Completer for RushCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.file_completer.complete(line, pos, ctx)
    }
}

#[derive(Helper, Completer, Hinter, Validator)]
pub struct RushHelper {
    #[rustyline(Completer)]
    pub completer: RushCompleter,
    #[rustyline(Hinter)]
    pub hinter: HistoryHinter,
    pub context: Context,
    pub path_checker: PathChecker,
}

#[derive(Debug, Clone, Copy)]
enum LexState {
    Command,
    Arg,
    Quoting,
}

// TODO : Store the paths to avoid recomputing it on every keystroke
pub struct PathChecker {}

impl PathChecker {
    fn valid_path_prefix(&self, prefix: &str) -> bool {
        self.valid_path_prefix_helper(prefix).unwrap_or(false)
    }
    fn valid_path_prefix_helper(&self, prefix: &str) -> Result<bool, ShellError> {
        let cwd = env::current_dir()?;

        // Get the part of the path before the last slash
        let (search_dir, file_prefix) = match prefix.rsplit_once('/') {
            Some((head, tail)) => (head, tail),
            None => ("", prefix),
        };

        let search_dir = PathBuf::from(search_dir);

        let target_dir = if search_dir.is_absolute() || search_dir.starts_with("~") {
            let expanded = format!("{}", shellexpand::tilde(search_dir.to_str().unwrap()));
            PathBuf::from(expanded)
        } else {
            cwd.join(search_dir)
        };

        let entries = fs::read_dir(target_dir)?;
        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name();

            match file_name.to_str() {
                Some(file_name) => {
                    if file_name.starts_with(file_prefix) {
                        return Ok(true);
                    }
                }
                None => continue,
            }
        }
        Ok(false)
    }
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
                    LexState::Command => match self.context.command_exists(slice) {
                        true => (slice.white().bold(), LexState::Arg),
                        false => (slice.bright_red(), LexState::Arg),
                    },
                    LexState::Arg => match self.path_checker.valid_path_prefix(slice) {
                        true => (slice.cyan().underline(), state),
                        false => (slice.cyan(), state),
                    },
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

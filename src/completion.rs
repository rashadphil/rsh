use std::path;

use colored::Colorize;
use rustyline::completion::{self, Completer, Pair};

use crate::parselex::{self, lex::Token};

// Highlight all candidates that start with the same prefix
fn highlight_candidates(candidates: &mut [Pair], prefix: &str) {
    let prefix = &prefix.to_lowercase();
    for candidate in candidates.iter_mut() {
        if candidate.display.to_lowercase().starts_with(prefix) {
            candidate.display = format!(
                "{}{}",
                &candidate.display[..prefix.len()].cyan().underline().bold(),
                &candidate.display[prefix.len()..]
            );
        }
    }
}

fn current_token(line: &str, pos: usize) -> Token {
    let lexed = parselex::lex(line);
    lexed
        .into_iter()
        .find(|(_token, span)| span.start <= pos && span.end >= pos)
        .map(|(token, _span)| token)
        .unwrap_or(Token::None)
}

#[derive(Default)]
pub struct FilenameCompleter {
    completer: completion::FilenameCompleter,
}

/// This completer contains all other completers
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

impl Completer for FilenameCompleter {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let (complete_pos, candidates) = self.completer.complete_path(line, pos)?;

        // remove dotfiles
        let mut candidates: Vec<Pair> = candidates
            .into_iter()
            .filter(|c| !c.replacement.starts_with('.'))
            .collect();

        // add trailing slash to directories
        for candidate in candidates.iter_mut() {
            if candidate.replacement.ends_with(path::MAIN_SEPARATOR) {
                candidate.display.push('/');
            }
        }

        let current_token = current_token(line, pos);
        let curr_word = match &current_token {
            Token::Item(i) => i,
            _ => "",
        };

        // Get the part of the path before the last slash
        let path_end = match curr_word.rsplit_once(path::MAIN_SEPARATOR) {
            Some((_, tail)) => tail,
            None => curr_word,
        };

        highlight_candidates(&mut candidates, path_end);
        Ok((complete_pos, candidates))
    }
}

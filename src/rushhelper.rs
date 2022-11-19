use rustyline::hint::HistoryHinter;
use rustyline::CompletionType;
use rustyline::{completion::FilenameCompleter, highlight::Highlighter};
use std::borrow::Cow::Owned;

use colored::*;
use rustyline_derive::{Completer, Helper, Hinter, Validator};

#[derive(Helper, Completer, Hinter, Validator)]
pub struct RushHelper {
    #[rustyline(Completer)]
    pub completer: FilenameCompleter,
    #[rustyline(Hinter)]
    pub hinter: HistoryHinter,
}

impl Highlighter for RushHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        let colored = hint.truecolor(140, 140, 140);
        Owned(format!("{}", colored))
    }

    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str,
        completion: CompletionType,
    ) -> std::borrow::Cow<'c, str> {
        let colored = candidate.white();
        Owned(format!("{}", colored))
    }
}

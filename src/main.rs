use std::borrow::Cow::Owned;

use commands::{Args, CommandType, ExternalCommand, InternalCommand};
use parser::ParsedCommand;
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::HistoryHinter;

use colored::*;
use rustyline::{Config, Editor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

use std::collections::BTreeMap;
use std::process;
use std::rc::Rc;
use types::primary::Value;

use environment::Environment;
use views::RenderView;

use crate::commands::Command;

use crate::types::primary::ToBaseView;

mod commands;
mod environment;
mod error;
mod parser;
mod types;
mod views;

#[derive(Default)]
pub struct Context {
    env: Rc<Environment>,
    valid_commands: BTreeMap<String, Rc<dyn Command>>,
}

#[derive(Helper, Completer, Hinter, Validator)]
struct RushHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Highlighter for RushHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        let colored = hint.truecolor(140, 140, 140);
        Owned(format!("{}", colored))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt_char = "➜ ";

    let mut context: Context = Context::default();

    let config = Config::builder().history_ignore_space(true).build();

    let mut rl = Editor::with_config(config)?;
    let h = RushHelper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter {},
    };
    rl.set_helper(Some(h));

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let ls = commands::ls::Ls;
    let ps = commands::ps::Ps;
    let cd = commands::cd::Cd;

    context.valid_commands.insert("ls".to_string(), Rc::new(ls));
    context.valid_commands.insert("ps".to_string(), Rc::new(ps));
    context.valid_commands.insert("cd".to_string(), Rc::new(cd));

    loop {
        let cwd = context.env.cwd();
        let truncated_cwd = cwd
            .file_name()
            .expect("Failed to read path")
            .to_string_lossy()
            .to_string();

        let readline = rl.readline(&format!(
            " {}\n {} ",
            truncated_cwd.cyan().bold(),
            prompt_char.red().bold()
        ));

        match process_readline(&context, readline) {
            LineResult::Success(line) => {
                rl.add_history_entry(line);
            }
            LineResult::Break => break,
            LineResult::Error(err) => println!("{}", err),
            LineResult::Fatal(fatal_err) => panic!("Fatal Error : {}", fatal_err),
        }
    }
    rl.save_history("history.txt").unwrap();
    Ok(())
}

enum LineResult {
    Success(String),
    Break,
    Error(String),
    Fatal(String),
}

fn process_readline(ctx: &Context, readline: Result<String, ReadlineError>) -> LineResult {
    match readline {
        Ok(line) => match line.as_str().trim() {
            "exit" => LineResult::Break,
            "" => LineResult::Success("".to_string()),
            _ => {
                let parsed_pipeline = parser::parse(&line);

                // TODO : add support for piping
                let first_command = &parsed_pipeline.commands[0];
                let command = parsed_to_command(ctx, first_command);
                match command {
                    CommandType::Internal(internal) => {
                        let command = internal.command;

                        let args = Args::new(ctx.env.clone(), internal.args);

                        let result = match command.run(args) {
                            Ok(res) => res,
                            Err(e) => return LineResult::Error(e.to_string()),
                        };

                        let base_view = result.to_base_view();
                        let rendered = base_view.render();
                        for line in rendered {
                            println!("{}", line);
                        }
                    }
                    CommandType::External(external) => {
                        let name = external.command;
                        let args = external.args;

                        let child = process::Command::new(&name).args(args).spawn();

                        match child {
                            Ok(mut child) => {
                                child.wait().unwrap();
                            }
                            Err(_) => println!("rush: command not found {}", name),
                        };
                    }
                }

                LineResult::Success(line)
            }
        },
        Err(ReadlineError::Interrupted) => LineResult::Success("".to_string()),
        Err(ReadlineError::Eof) => LineResult::Break,
        Err(err) => LineResult::Fatal(err.to_string()),
    }
}

fn parsed_to_command(ctx: &Context, parsed_command: &ParsedCommand) -> CommandType {
    let name = &parsed_command.name;
    let args = &parsed_command.args;

    if let Some(command) = ctx.valid_commands.get(name) {
        let command = command.clone();
        let args: Vec<Value> = args.iter().map(|arg| arg.to_value()).collect();

        let internal_command = InternalCommand::new(command, args);
        CommandType::Internal(internal_command)
    } else {
        let name = name.to_string();
        let args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        let external_command = ExternalCommand::new(name, args);
        CommandType::External(external_command)
    }
}

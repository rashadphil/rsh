use ansi_term::Color;
use commands::{Args, CommandType, ExternalCommand, InternalCommand};
use parser::ParsedCommand;
use std::collections::BTreeMap;
use std::process;
use std::rc::Rc;
use types::primary::Value;

use environment::Environment;
use rustyline::error::ReadlineError;
use rustyline::Editor;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt_char = "➜ ";

    let mut context: Context = Context::default();

    let mut rl = Editor::<()>::new()?;
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
            Color::Cyan.bold().paint(truncated_cwd),
            Color::Red.bold().paint(prompt_char)
        ));

        match process_readline(&context, readline) {
            LineResult::Success => continue,
            LineResult::Break => break,
            LineResult::Error(err) => println!("{}", err),
            LineResult::Fatal(fatal_err) => panic!("Fatal Error : {}", fatal_err),
        }
    }
    rl.save_history("history.txt").unwrap();
    Ok(())
}

enum LineResult {
    Success,
    Break,
    Error(String),
    Fatal(String),
}

fn process_readline(ctx: &Context, readline: Result<String, ReadlineError>) -> LineResult {
    match readline {
        Ok(line) => match line.as_str().trim() {
            "exit" => LineResult::Break,
            "" => LineResult::Success,
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

                LineResult::Success
            }
        },
        Err(ReadlineError::Interrupted) => LineResult::Success,
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

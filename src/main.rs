use ansi_term::Color;
use commands::{CommandType, ExternalCommand, InternalCommand};
use parser::ParsedCommand;
use std::collections::BTreeMap;
use std::process;
use std::rc::Rc;

use environment::Environment;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
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
    env: Environment,
    valid_commands: BTreeMap<String, Rc<dyn Command>>,
}

fn main() -> Result<()> {
    let PROMPT_CHAR = "➜ ";

    let mut context: Context = Context::default();

    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let ls = commands::ls::Ls {};
    let ps = commands::ps::Ps::new(sysinfo::System::default());

    context.valid_commands.insert("ls".to_string(), Rc::new(ls));
    context.valid_commands.insert("ps".to_string(), Rc::new(ps));

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
            Color::Red.bold().paint(PROMPT_CHAR)
        ));
        match readline {
            Ok(line) => {
                let line = line.trim().to_string();

                let parsed_pipeline = parser::parse(&line);

                // TODO : add support for piping
                let first_command = &parsed_pipeline.commands[0];
                let command = parsed_to_command(&context, first_command);

                // run the command
                match command {
                    CommandType::Internal(internal) => {
                        let command = internal.command;
                        let args = internal.args;

                        let result = command.run(args).unwrap();
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
                                child.wait();
                            }
                            Err(_) => println!("rush: command not found {}", name),
                        };
                    }
                }

                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}

fn parsed_to_command(ctx: &Context, parsed_command: &ParsedCommand) -> CommandType {
    let name = &parsed_command.name;
    let args = &parsed_command.args;

    match ctx.valid_commands.get(name) {
        Some(command) => {
            let command = command.clone();

            // TODO : add parsed arguments to internal command
            let internal_command = InternalCommand::new(command, vec![]);
            CommandType::Internal(internal_command)
        }
        None => {
            let name = name.to_string();
            let args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
            let external_command = ExternalCommand::new(name, args);
            CommandType::External(external_command)
        }
    }
}

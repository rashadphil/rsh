use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::hint::HistoryHinter;

use colored::*;
use rustyline::{CompletionType, Config, Editor};

use std::process::Stdio;
use std::rc::Rc;

use crate::commands::{self, CommandType, ExternalCommand, InternalCommand};
use crate::parser::{self, ParsedCommand, ParsedPipeline};
use crate::rushhelper::RushHelper;

use crate::stream::InStream;
use crate::types::primary::{ToBaseView, Value};

use crate::context::Context;
use crate::views::RenderView;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let prompt_char = "➜ ";

    let mut context = Context::default();

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::CircularList)
        .auto_add_history(true)
        .max_history_size(1000)
        .build();

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
    let sortby = commands::sortby::SortBy;
    let limit = commands::limit::Limit;

    context.insert_commands(vec![
        ("ls", Rc::new(ls)),
        ("ps", Rc::new(ps)),
        ("cd", Rc::new(cd)),
        ("sortby", Rc::new(sortby)),
        ("limit", Rc::new(limit)),
    ]);

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
            LineResult::Success(_) => continue,
            LineResult::Break => break,
            LineResult::Error(err) => println!("{}", err),
            LineResult::Fatal(fatal_err) => panic!("Fatal Error : {}", fatal_err),
        }
    }

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
                let command_list = build_pipeline(ctx, &parsed_pipeline);

                let mut pipeline_iter = command_list.into_iter().peekable();

                let (curr, next) = (pipeline_iter.next(), pipeline_iter.next());

                match (curr, next) {
                    (None, None) => unreachable!(),
                    (None, Some(_)) => unreachable!(),
                    (Some(last_command), None) => match last_command {
                        CommandType::Internal(internal) => {
                            let result = match internal.run(ctx, None) {
                                Ok(val) => val,
                                Err(e) => return LineResult::Error(e.to_string()),
                            };
                            let base_view = result.to_base_view();
                            let rendered = base_view.render();
                            for line in rendered {
                                println!("{}", line);
                            }
                        }
                        CommandType::External(external) => {
                            let mut child = match external.run(Stdio::inherit(), Stdio::inherit()) {
                                Ok(child) => child,
                                Err(_) => {
                                    return LineResult::Error(format!(
                                        "rush : command not found {}",
                                        external.command
                                    ))
                                }
                            };
                            child.wait();
                        }
                    },
                    (Some(first_command), Some(second_command)) => {
                        match (first_command, second_command) {
                            (CommandType::Internal(first), CommandType::Internal(second)) => {
                                let result = match first.run(ctx, None) {
                                    Ok(val) => val,
                                    Err(e) => return LineResult::Error(e.to_string()),
                                };

                                let input_stream = InStream::new(result);

                                let result2 = match second.run(ctx, Some(input_stream)) {
                                    Ok(val) => val,
                                    Err(e) => return LineResult::Error(e.to_string()),
                                };

                                let base_view = result2.to_base_view();
                                let rendered = base_view.render();
                                for line in rendered {
                                    println!("{}", line);
                                }
                            }
                            (CommandType::Internal(_), CommandType::External(_)) => {
                                return LineResult::Error(
                                    "Internal to External Pipe not yet implemented!".to_string(),
                                )
                            }
                            (CommandType::External(_), CommandType::Internal(_)) => {
                                return LineResult::Error(
                                    "External to Internal Pipe not yet implemented!".to_string(),
                                )
                            }
                            (CommandType::External(first), CommandType::External(second)) => {
                                let child_one = match first.run(Stdio::inherit(), Stdio::piped()) {
                                    Ok(child) => child,
                                    Err(e) => return LineResult::Error(e.to_string()),
                                };
                                let mut child_two = match second
                                    .run(Stdio::from(child_one.stdout.unwrap()), Stdio::inherit())
                                {
                                    Ok(child) => child,
                                    Err(e) => return LineResult::Error(e.to_string()),
                                };

                                child_two.wait();
                            }
                        }
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

fn build_pipeline(ctx: &Context, parsed_pipeline: &ParsedPipeline) -> Vec<CommandType> {
    let commands = &parsed_pipeline.commands;

    commands
        .iter()
        .map(|command| parsed_to_command(ctx, command))
        .collect()
}

fn parsed_to_command(ctx: &Context, parsed_command: &ParsedCommand) -> CommandType {
    let name = &parsed_command.name;
    let args = &parsed_command.args;

    if let Some(command) = ctx.valid_commands.get(name) {
        let command = command.clone();
        let args: Vec<Value> = args.iter().map(Value::from).collect();

        let internal_command = InternalCommand::new(command, args);
        CommandType::Internal(internal_command)
    } else {
        let name = name.to_string();
        let args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        let external_command = ExternalCommand::new(name, args);
        CommandType::External(external_command)
    }
}

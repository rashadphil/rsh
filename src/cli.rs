use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::hint::HistoryHinter;

use colored::*;
use rustyline::{CompletionType, Config, Editor};

use std::process::Stdio;
use std::rc::Rc;

use crate::commands::{self, CommandType, ExternalCommand, InternalCommand};
use crate::error::ShellError;
use crate::parselex;
use crate::parselex::parser::{ParsedCommand, ParsedPipeline};
use crate::rushhelper::RushHelper;

use crate::stream::RushStream;
use crate::types::primary::{ToBaseView, Value};

use crate::context::Context;
use crate::views::RenderView;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let prompt_char = "➜";
    let branch_char = " ";

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
    let take = commands::take::Take;
    let rev = commands::rev::Rev;

    context.insert_commands(vec![
        ("ls", Rc::new(ls)),
        ("ps", Rc::new(ps)),
        ("cd", Rc::new(cd)),
        ("sortby", Rc::new(sortby)),
        ("take", Rc::new(take)),
        ("rev", Rc::new(rev)),
    ]);

    loop {
        let cwd = context.env.cwd();

        let repo = git_repository::discover(&cwd).ok();

        let branch_name = match repo {
            Some(repo) => {
                let head = repo.head_name()?;
                head.map(|head| head.shorten().to_string())
            }
            None => None,
        };

        let branch_str = match branch_name {
            Some(branch_name) => format!(
                "on {}{}",
                branch_char.purple().bold(),
                branch_name.purple().bold()
            )
            .to_string(),
            None => "".to_string(),
        };

        let truncated_cwd = cwd
            .file_name()
            .expect("Failed to read path")
            .to_string_lossy()
            .to_string();

        let readline = rl.readline(&format!(
            " {} {} \n {} ",
            truncated_cwd.cyan().bold(),
            branch_str,
            prompt_char.red().bold()
        ));

        match process_readline(&context, readline) {
            Ok(line_res) => match line_res {
                LineResult::Success(val) => {
                    let base_view = val.to_base_view();
                    let rendered = base_view.render();
                    for line in rendered {
                        println!("{}", line);
                    }
                }
                LineResult::Break => break,
                LineResult::Error(err) => println!("{}", err),
                LineResult::Fatal(fatal_err) => panic!("Fatal Error : {}", fatal_err),
            },
            Err(err) => println!("Error: {}", err),
        }
    }

    rl.save_history("history.txt")?;

    Ok(())
}

enum LineResult {
    Success(Value),
    Break,
    Error(String),
    Fatal(String),
}

fn process_readline(
    ctx: &Context,
    readline: Result<String, ReadlineError>,
) -> Result<LineResult, ShellError> {
    match readline {
        Ok(line) => match line.as_str().trim() {
            "exit" => Ok(LineResult::Break),
            "" => Ok(LineResult::Success(Value::none())),
            _ => {
                let parsed_pipeline = parselex::parser::parse(&line);
                let command_list = build_pipeline(ctx, &parsed_pipeline);

                let mut pipeline_iter = command_list.into_iter().peekable();

                let mut stream = RushStream::None;

                let final_result = loop {
                    let (curr, next) = (pipeline_iter.next(), pipeline_iter.peek());

                    stream = match (curr, next) {
                        (Some(final_command), None) => match final_command {
                            CommandType::Internal(internal) => {
                                let result = internal.run(ctx, stream)?;
                                break result;
                            }
                            CommandType::External(external) => {
                                let mut result = external.run(stream, Stdio::inherit())?;
                                result.wait()?;
                                break Value::none();
                            }
                        },
                        (Some(curr_command), Some(_)) => match curr_command {
                            CommandType::Internal(internal) => {
                                let result = internal.run(ctx, stream)?;
                                RushStream::Internal(result)
                            }
                            CommandType::External(external) => {
                                let result = external.run(stream, Stdio::piped())?;
                                RushStream::External(result.stdout.unwrap().into())
                            }
                        },
                        (_, _) => return Ok(LineResult::Error("Not yet implemented".to_string())),
                    }
                };
                Ok(LineResult::Success(final_result))
            }
        },
        Err(ReadlineError::Interrupted) => Ok(LineResult::Success(Value::none())),
        Err(ReadlineError::Eof) => Ok(LineResult::Break),
        Err(err) => Ok(LineResult::Fatal(err.to_string())),
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

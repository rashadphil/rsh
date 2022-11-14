use std::collections::BTreeMap;

use crate::commands::Command;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

mod commands;
mod error;
mod types;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut valid_commands = BTreeMap::<String, Box<dyn Command>>::new();
    let ls = commands::ls::Ls {};
    let ps = commands::ps::Ps {};
    valid_commands.insert("ls".to_string(), Box::new(ls));
    valid_commands.insert("ps".to_string(), Box::new(ps));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let _ = match valid_commands.get(&line) {
                    Some(command) => {
                        let result = command.run();
                        println!("Result : {:?}", result);
                    }
                    None => {
                        println!("Line: {}", line);
                    }
                };
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

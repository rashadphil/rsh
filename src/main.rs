use std::collections::BTreeMap;

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use views::RenderView;

use crate::commands::Command;
use crate::types::primary::ToBaseView;

mod commands;
mod error;
mod types;
mod views;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut valid_commands = BTreeMap::<String, Box<dyn Command>>::new();
    let ls = commands::ls::Ls {};
    let ps = commands::ps::Ps::new(sysinfo::System::default());
    valid_commands.insert("ls".to_string(), Box::new(ls));
    valid_commands.insert("ps".to_string(), Box::new(ps));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let _ = match valid_commands.get_mut(&line) {
                    Some(command) => {
                        let result = command.run().unwrap();
                        let base_view = result.to_base_view();
                        let rendered = base_view.render();
                        for line in rendered {
                            println!("{}", line);
                        }
                        // println!("Result : {:?}", result);
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

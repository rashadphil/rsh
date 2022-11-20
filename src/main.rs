mod cli;
mod commands;
mod completion;
mod context;
mod environment;
mod error;
mod highlight;
mod parselex;
mod rushhelper;
mod stream;
mod types;
mod views;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run()
}

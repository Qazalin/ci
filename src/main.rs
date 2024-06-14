use clap::{command, Parser, Subcommand};

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Start,
    Ls,
    Watch,
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}

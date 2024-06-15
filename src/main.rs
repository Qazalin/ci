use clap::{command, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    Start(StartArgs),
    Ls,
    Check(CheckArgs),
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
pub struct StartArgs {
    #[arg(short, help = "Add a string parameter in key=value format", long)]
    field: String,
}

#[derive(Parser, Debug)]
pub struct CheckArgs {
    #[arg(short, help = "Poll output every 10s", long)]
    watch: bool,
    #[arg(short, help = "Notify once done", long)]
    notify: bool,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Start(_) => todo!(),
        Command::Ls => todo!(),
        Command::Check(_) => todo!(),
    }
}

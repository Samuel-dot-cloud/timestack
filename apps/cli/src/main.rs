use clap::Parser;
use crate::commands::{execute_command, Commands};

mod db;
mod tracking;
mod sync;
mod commands;

#[derive(Parser)]
#[command(name = "timestack-cli")]
#[command(version = "1.0.0")]
#[command(about = "Helper CLI to be used by timestack editor plugins")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    execute_command(cli.command)
}

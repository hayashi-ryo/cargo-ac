mod cli;
mod commands;

use clap::Parser;

fn main() -> commands::CommandResult {
    let cli = cli::Cli::parse();
    cli::dispatch(cli.command)
}

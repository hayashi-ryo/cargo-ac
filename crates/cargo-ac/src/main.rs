mod cli;
mod commands;
mod error;

use std::process::ExitCode;

use clap::Parser;

use error::CliResult;

fn main() -> ExitCode {
    handle_result(run())
}

fn run() -> CliResult {
    let cli = cli::Cli::parse();
    cli::dispatch(cli.command)
}

fn handle_result(result: CliResult) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io, process::ExitCode};

    use super::handle_result;

    #[test]
    fn returns_failure_exit_code_for_error() {
        let error = io::Error::other("test error");

        assert_eq!(handle_result(Err(Box::new(error))), ExitCode::FAILURE);
    }
}

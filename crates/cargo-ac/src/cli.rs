use clap::{Parser, Subcommand};

use crate::{commands, error::CliResult};

#[derive(Parser)]
#[command(bin_name = "cargo ac")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    New {
        contest: String,
    },
    Download {
        contest: String,
    },
    Test {
        task: String,
        #[arg(long)]
        release: bool,
    },
    Addcase {
        task: String,
    },
    Login,
    Submit {
        task: String,
        #[arg(long)]
        watch: bool,
    },
    Watch,
    Doctor,
    Selfcheck,
    Env {
        #[command(subcommand)]
        command: EnvCommand,
    },
    Lang {
        #[command(subcommand)]
        command: LangCommand,
    },
}

#[derive(Subcommand)]
pub(crate) enum EnvCommand {
    Show,
    Update,
}

#[derive(Subcommand)]
pub(crate) enum LangCommand {
    Refresh,
}

pub(crate) fn dispatch(command: Command) -> CliResult {
    match command {
        Command::New { contest } => commands::new::run(contest),
        Command::Download { contest } => commands::download::run(contest),
        Command::Test { task, release } => commands::test::run(task, release),
        Command::Addcase { task } => commands::addcase::run(task),
        Command::Login => commands::login::run(),
        Command::Submit { task, watch } => commands::submit::run(task, watch),
        Command::Watch => commands::watch::run(),
        Command::Doctor => commands::doctor::run(),
        Command::Selfcheck => commands::selfcheck::run(),
        Command::Env { command } => dispatch_env(command),
        Command::Lang { command } => dispatch_lang(command),
    }
}

fn dispatch_env(command: EnvCommand) -> CliResult {
    match command {
        EnvCommand::Show => commands::env::show(),
        EnvCommand::Update => commands::env::update(),
    }
}

fn dispatch_lang(command: LangCommand) -> CliResult {
    match command {
        LangCommand::Refresh => commands::lang::refresh(),
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::Parser;

    #[test]
    fn parses_initial_command_shapes() {
        let commands = [
            &["cargo-ac", "new", "abc400"][..],
            &["cargo-ac", "download", "abc400"],
            &["cargo-ac", "test", "a"],
            &["cargo-ac", "test", "a", "--release"],
            &["cargo-ac", "test", "all", "--release"],
            &["cargo-ac", "addcase", "a"],
            &["cargo-ac", "login"],
            &["cargo-ac", "submit", "a"],
            &["cargo-ac", "submit", "a", "--watch"],
            &["cargo-ac", "watch"],
            &["cargo-ac", "doctor"],
            &["cargo-ac", "selfcheck"],
            &["cargo-ac", "env", "show"],
            &["cargo-ac", "env", "update"],
            &["cargo-ac", "lang", "refresh"],
        ];

        for command in commands {
            assert!(Cli::try_parse_from(command).is_ok(), "{command:?}");
        }
    }
}

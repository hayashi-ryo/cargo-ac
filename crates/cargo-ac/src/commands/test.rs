use crate::error::CliResult;

pub(crate) fn run(_task: String) -> CliResult {
    println!("`cargo ac test` is not implemented yet.");
    Ok(())
}

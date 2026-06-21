use crate::error::CliResult;

pub(crate) fn run(_task: String, _watch: bool) -> CliResult {
    println!("`cargo ac submit` is not implemented yet.");
    Ok(())
}

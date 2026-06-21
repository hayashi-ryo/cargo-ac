use std::error::Error;

pub(crate) type CliResult = Result<(), Box<dyn Error>>;

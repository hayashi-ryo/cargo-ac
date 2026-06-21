use std::error::Error;

pub(crate) mod addcase;
pub(crate) mod doctor;
pub(crate) mod download;
pub(crate) mod env;
pub(crate) mod lang;
pub(crate) mod login;
pub(crate) mod new;
pub(crate) mod selfcheck;
pub(crate) mod submit;
pub(crate) mod test;
pub(crate) mod watch;

pub(crate) type CommandResult = Result<(), Box<dyn Error>>;

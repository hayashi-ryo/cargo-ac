use std::{env, path::Path};

use ac_core::{
    config::{ProjectConfig, TaskConfig},
    manifest::write_manifest,
    task_layout::generate_task_layout,
    workspace::{create_workspace, WorkspaceRequest},
};

use crate::error::CliResult;

const TASK_NAMES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];
const RUST_EDITION: &str = "2021";

pub(crate) fn run(contest: String) -> CliResult {
    let current_directory = env::current_dir()?;
    generate_workspace(&current_directory, &contest)?;
    println!("Created contest workspace `{contest}`.");
    Ok(())
}

fn generate_workspace(parent: &Path, contest: &str) -> CliResult {
    let tasks = TASK_NAMES
        .into_iter()
        .map(|name| TaskConfig::new(format!("{contest}_{name}"), name))
        .collect::<Vec<_>>();
    let request = WorkspaceRequest::new(parent.join(contest), contest, tasks.clone());
    let paths = create_workspace(&request)?;

    write_manifest(paths.manifest(), request.contest_id(), RUST_EDITION)?;
    ProjectConfig::new(
        request.contest_id(),
        "src/bin",
        "testcases",
        "rust",
        RUST_EDITION,
        tasks,
    )
    .write(paths.config())?;
    generate_task_layout(&request, &paths)?;

    Ok(())
}

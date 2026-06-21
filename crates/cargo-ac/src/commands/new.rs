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

#[cfg(test)]
mod tests {
    use std::{fs, process::Command};

    use ac_core::config::ProjectConfig;
    use tempfile::tempdir;

    use super::generate_workspace;

    #[test]
    fn generates_complete_contest_workspace() {
        let directory = tempdir().expect("temporary directory should be created");

        generate_workspace(directory.path(), "abc400").expect("workspace should be generated");

        let workspace = directory.path().join("abc400");
        assert!(workspace.join("Cargo.toml").is_file());
        let config = ProjectConfig::read(workspace.join("ac.toml"))
            .expect("generated config should be readable");
        assert_eq!(config.contest_id(), "abc400");
        assert_eq!(config.tasks().len(), 6);

        for name in ["a", "b", "c", "d", "e", "f"] {
            assert!(workspace.join(format!("src/bin/{name}.rs")).is_file());
            assert!(workspace.join(format!("testcases/{name}")).is_dir());
        }

        let output = Command::new(env!("CARGO"))
            .args(["check", "--bins", "--offline", "--manifest-path"])
            .arg(workspace.join("Cargo.toml"))
            .output()
            .expect("cargo check should run");
        assert!(
            output.status.success(),
            "cargo check failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn does_not_modify_existing_destination() {
        let directory = tempdir().expect("temporary directory should be created");
        let workspace = directory.path().join("abc400");
        fs::create_dir(&workspace).expect("existing workspace should be created");
        let marker = workspace.join("existing.txt");
        fs::write(&marker, "existing").expect("marker should be written");

        generate_workspace(directory.path(), "abc400")
            .expect_err("existing workspace should be rejected");

        assert_eq!(
            fs::read_to_string(marker).expect("marker should remain readable"),
            "existing"
        );
        assert_eq!(
            fs::read_dir(workspace)
                .expect("workspace should be readable")
                .count(),
            1
        );
    }

    #[test]
    fn rejects_invalid_contest_id_without_writing_outside_parent() {
        let directory = tempdir().expect("temporary directory should be created");
        let outside_name = format!(
            "{}_outside",
            directory
                .path()
                .file_name()
                .expect("temporary directory should have a name")
                .to_string_lossy()
        );
        let outside = directory
            .path()
            .parent()
            .expect("temporary directory should have a parent")
            .join(&outside_name);
        let contest = format!("../{outside_name}");

        generate_workspace(directory.path(), &contest)
            .expect_err("invalid contest ID should be rejected");

        assert!(!outside.exists());
    }
}

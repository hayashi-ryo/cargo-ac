use std::{fs, path::Path, process::Command};

use ac_core::config::ProjectConfig;
use tempfile::tempdir;

const TASK_NAMES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

fn cargo_ac(current_directory: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-ac"));
    command.current_dir(current_directory);
    command
}

#[test]
fn generates_complete_buildable_contest_workspace() {
    let directory = tempdir().expect("temporary directory should be created");

    let output = cargo_ac(directory.path())
        .args(["new", "abc400"])
        .output()
        .expect("cargo-ac should run");

    assert!(
        output.status.success(),
        "cargo ac new failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Created contest workspace `abc400`"));

    let workspace = directory.path().join("abc400");
    let manifest = fs::read_to_string(workspace.join("Cargo.toml"))
        .expect("generated manifest should be readable");
    assert!(manifest.contains("name = \"abc400\""));
    assert!(manifest.contains("edition = \"2021\""));

    let config =
        ProjectConfig::read(workspace.join("ac.toml")).expect("generated config should be valid");
    assert_eq!(config.contest_id(), "abc400");
    assert_eq!(config.source_directory(), Path::new("src/bin"));
    assert_eq!(config.testcase_directory(), Path::new("testcases"));
    assert_eq!(config.language(), "rust");
    assert_eq!(config.rust_edition(), "2021");
    assert_eq!(config.tasks().len(), TASK_NAMES.len());

    for (task, name) in config.tasks().iter().zip(TASK_NAMES) {
        assert_eq!(task.task_id(), format!("abc400_{name}"));
        assert_eq!(task.bin_name(), name);

        let source = workspace.join(format!("src/bin/{name}.rs"));
        assert!(source.is_file());
        assert!(fs::read_to_string(source)
            .expect("generated source should be readable")
            .contains("read_to_string"));

        let testcase_directory = workspace.join(format!("testcases/{name}"));
        assert!(testcase_directory.is_dir());
        assert_eq!(
            fs::read_dir(testcase_directory)
                .expect("testcase directory should be readable")
                .count(),
            0
        );
    }

    let build = Command::new(env!("CARGO"))
        .args(["check", "--bins", "--offline", "--manifest-path"])
        .arg(workspace.join("Cargo.toml"))
        .output()
        .expect("cargo check should run");
    assert!(
        build.status.success(),
        "generated workspace failed to build: {}",
        String::from_utf8_lossy(&build.stderr)
    );
}

#[test]
fn rejects_existing_workspace_without_modifying_it() {
    let directory = tempdir().expect("temporary directory should be created");
    let workspace = directory.path().join("abc400");
    fs::create_dir(&workspace).expect("existing workspace should be created");
    let marker = workspace.join("existing.txt");
    fs::write(&marker, "existing").expect("marker should be written");

    let output = cargo_ac(directory.path())
        .args(["new", "abc400"])
        .output()
        .expect("cargo-ac should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("already exists"));
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
fn rejects_invalid_contest_id_without_writing_outside_workspace() {
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

    let output = cargo_ac(directory.path())
        .args(["new", &contest])
        .output()
        .expect("cargo-ac should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid contest ID"));
    assert!(!outside.exists());
    assert_eq!(
        fs::read_dir(directory.path())
            .expect("temporary directory should be readable")
            .count(),
        0
    );
}

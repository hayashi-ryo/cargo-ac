use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use ac_core::config::{ProjectConfig, TaskConfig};
use tempfile::tempdir;

fn cargo_ac(current_directory: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-ac"));
    command.current_dir(current_directory);
    command
}

fn run_with_stdin(mut command: Command, stdin: &[u8]) -> std::process::Output {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("cargo-ac should spawn");
    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(stdin)
        .expect("stdin should be written");
    child.wait_with_output().expect("cargo-ac should finish")
}

fn write_workspace(root: &Path) {
    fs::create_dir_all(root.join("testcases/a")).expect("task a directory should be created");
    fs::create_dir_all(root.join("testcases/b")).expect("task b directory should be created");
    let config = ProjectConfig::new(
        "abc400",
        "src/bin",
        "testcases",
        "rust",
        "2021",
        vec![
            TaskConfig::new("abc400_a", "a"),
            TaskConfig::new("abc400_b", "b"),
        ],
    );
    config
        .write(root.join("ac.toml"))
        .expect("config should be written");
}

#[test]
fn adds_custom_testcase_from_interactive_blocks() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path());
    let mut command = cargo_ac(directory.path());
    command.args(["addcase", "a"]);

    let output = run_with_stdin(command, b"1 2\n---\n3\n---\n");

    assert!(
        output.status.success(),
        "addcase failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Added testcase `custom-1`"));
    assert_eq!(
        fs::read_to_string(directory.path().join("testcases/a/custom-1.in"))
            .expect("input should be readable"),
        "1 2\n"
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("testcases/a/custom-1.out"))
            .expect("output should be readable"),
        "3\n"
    );
}

#[test]
fn skips_existing_custom_numbers_without_overwriting() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path());
    fs::write(
        directory.path().join("testcases/a/custom-1.out"),
        "existing\n",
    )
    .expect("partial existing output should be written");
    let mut command = cargo_ac(directory.path());
    command.args(["addcase", "a"]);

    let output = run_with_stdin(command, b"in\n---\nout\n---\n");

    assert!(
        output.status.success(),
        "addcase failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("testcases/a/custom-1.out"))
            .expect("existing output should remain"),
        "existing\n"
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("testcases/a/custom-2.in"))
            .expect("new input should be readable"),
        "in\n"
    );
    assert_eq!(
        fs::read_to_string(directory.path().join("testcases/a/custom-2.out"))
            .expect("new output should be readable"),
        "out\n"
    );
}

#[test]
fn rejects_unknown_task_with_available_tasks() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path());
    let mut command = cargo_ac(directory.path());
    command.args(["addcase", "missing"]);

    let output = run_with_stdin(command, b"in\n---\nout\n---\n");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown task `missing`"));
    assert!(stderr.contains("available tasks: a, b"));
}

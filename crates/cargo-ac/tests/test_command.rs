use std::{fs, path::Path, process::Command};

use ac_core::config::{ProjectConfig, TaskConfig};
use tempfile::tempdir;

fn cargo_ac(current_directory: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-ac"));
    command.current_dir(current_directory);
    command
}

fn write_workspace(root: &Path, tasks: &[(&str, &str)]) {
    fs::write(
        root.join("Cargo.toml"),
        r#"[package]
name = "test_fixture"
version = "0.1.0"
edition = "2021"
"#,
    )
    .expect("manifest should be written");
    fs::create_dir_all(root.join("src/bin")).expect("bin directory should be created");
    fs::create_dir_all(root.join("testcases")).expect("testcase directory should be created");

    let config = ProjectConfig::new(
        "test_fixture",
        "src/bin",
        "testcases",
        "rust",
        "2021",
        tasks
            .iter()
            .map(|(task_id, bin_name)| TaskConfig::new(*task_id, *bin_name))
            .collect(),
    );
    config
        .write(root.join("ac.toml"))
        .expect("config should be written");
}

fn write_bin(root: &Path, name: &str, source: &str) {
    fs::write(root.join(format!("src/bin/{name}.rs")), source).expect("source should be written");
}

fn write_case(root: &Path, task: &str, name: &str, input: &str, output: &str) {
    let directory = root.join(format!("testcases/{task}"));
    fs::create_dir_all(&directory).expect("task testcase directory should be created");
    fs::write(directory.join(format!("{name}.in")), input).expect("input should be written");
    fs::write(directory.join(format!("{name}.out")), output).expect("output should be written");
}

#[test]
fn runs_single_task_testcases_successfully() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path(), &[("fixture_a", "a")]);
    write_bin(
        directory.path(),
        "a",
        r#"use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    print!("{}", input);
}
"#,
    );
    write_case(directory.path(), "a", "sample1", "hello\n", "hello\n");

    let output = cargo_ac(directory.path())
        .args(["test", "a"])
        .output()
        .expect("cargo-ac should run");

    assert!(
        output.status.success(),
        "test command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("task a"));
    assert!(stdout.contains("[AC] sample1"));
    assert!(stdout.contains("summary: AC 1 WA 0 RE 0 TLE 0 total 1"));
}

#[test]
fn runs_all_tasks_in_config_order_and_reports_failure_status() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path(), &[("fixture_a", "a"), ("fixture_b", "b")]);
    write_bin(directory.path(), "a", "fn main() { println!(\"ok\"); }");
    write_bin(directory.path(), "b", "fn main() { println!(\"ng\"); }");
    write_case(directory.path(), "a", "sample1", "", "ok\n");
    write_case(directory.path(), "b", "sample1", "", "ok\n");

    let output = cargo_ac(directory.path())
        .args(["test", "all", "--release"])
        .output()
        .expect("cargo-ac should run");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let task_a = stdout.find("task a").expect("task a should be displayed");
    let task_b = stdout.find("task b").expect("task b should be displayed");
    assert!(task_a < task_b);
    assert!(stdout.contains("[AC] sample1"));
    assert!(stdout.contains("[WA] sample1"));
    assert!(stdout.contains("expected:"));
    assert!(String::from_utf8_lossy(&output.stderr).contains("one or more testcases failed"));
}

#[test]
fn passes_release_profile_to_runner() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path(), &[("fixture_a", "a")]);
    write_bin(
        directory.path(),
        "a",
        r#"fn main() {
    if cfg!(debug_assertions) {
        println!("debug");
    } else {
        println!("release");
    }
}
"#,
    );
    write_case(directory.path(), "a", "sample1", "", "release\n");

    let output = cargo_ac(directory.path())
        .args(["test", "a", "--release"])
        .output()
        .expect("cargo-ac should run");

    assert!(
        output.status.success(),
        "release test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("summary: AC 1 WA 0 RE 0 TLE 0 total 1")
    );
}

#[test]
fn rejects_unknown_task_with_available_tasks() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path(), &[("fixture_a", "a"), ("fixture_b", "b")]);

    let output = cargo_ac(directory.path())
        .args(["test", "missing"])
        .output()
        .expect("cargo-ac should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown task `missing`"));
    assert!(stderr.contains("available tasks: a, b"));
}

#[test]
fn treats_zero_testcases_as_failure() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(directory.path(), &[("fixture_a", "a")]);
    write_bin(directory.path(), "a", "fn main() {}");
    fs::create_dir_all(directory.path().join("testcases/a"))
        .expect("empty testcase directory should be created");

    let output = cargo_ac(directory.path())
        .args(["test", "a"])
        .output()
        .expect("cargo-ac should run");

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("summary: AC 0 WA 0 RE 0 TLE 0 total 0")
    );
}

#[test]
fn treats_all_as_reserved_selector() {
    let directory = tempdir().expect("temporary directory should be created");
    write_workspace(
        directory.path(),
        &[("fixture_all", "all"), ("fixture_a", "a")],
    );
    write_bin(directory.path(), "all", "fn main() { println!(\"ok\"); }");
    write_bin(directory.path(), "a", "fn main() { println!(\"ok\"); }");
    write_case(directory.path(), "all", "sample1", "", "ok\n");
    write_case(directory.path(), "a", "sample1", "", "ok\n");

    let output = cargo_ac(directory.path())
        .args(["test", "all"])
        .output()
        .expect("cargo-ac should run");

    assert!(
        output.status.success(),
        "reserved all selector failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("task all"));
    assert!(stdout.contains("task a"));
}

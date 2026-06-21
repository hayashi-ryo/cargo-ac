use std::{
    collections::HashSet,
    error::Error,
    fmt, fs,
    fs::OpenOptions,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::workspace::{WorkspacePaths, WorkspaceRequest};

const DEFAULT_SOURCE: &str = r#"use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
}
"#;

pub fn generate_task_layout(
    request: &WorkspaceRequest,
    paths: &WorkspacePaths,
) -> Result<(), TaskLayoutError> {
    validate_tasks(request)?;
    ensure_path_is_available(paths.source_directory())?;
    ensure_path_is_available(paths.testcase_directory())?;

    create_directory_all(paths.source_directory())?;
    create_directory(paths.testcase_directory())?;

    for task in request.tasks() {
        let source = paths
            .source_directory()
            .join(format!("{}.rs", task.bin_name()));
        write_source(&source)?;

        let testcase_directory = paths.testcase_directory().join(task.bin_name());
        create_directory(&testcase_directory)?;
    }

    Ok(())
}

fn validate_tasks(request: &WorkspaceRequest) -> Result<(), TaskLayoutError> {
    let mut names = HashSet::new();

    for task in request.tasks() {
        let name = task.bin_name();
        let mut characters = name.chars();
        let valid_first = characters
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic() || character == '_');
        let valid_remaining =
            characters.all(|character| character.is_ascii_alphanumeric() || character == '_');

        if !valid_first || !valid_remaining {
            return Err(TaskLayoutError::InvalidTaskName {
                name: name.to_owned(),
            });
        }

        if !names.insert(name) {
            return Err(TaskLayoutError::DuplicateTaskName {
                name: name.to_owned(),
            });
        }
    }

    Ok(())
}

fn ensure_path_is_available(path: &Path) -> Result<(), TaskLayoutError> {
    match path.try_exists() {
        Ok(true) => Err(TaskLayoutError::PathExists {
            path: path.to_path_buf(),
        }),
        Ok(false) => Ok(()),
        Err(source) => Err(TaskLayoutError::InspectPath {
            path: path.to_path_buf(),
            source,
        }),
    }
}

fn create_directory_all(path: &Path) -> Result<(), TaskLayoutError> {
    fs::create_dir_all(path).map_err(|source| TaskLayoutError::CreateDirectory {
        path: path.to_path_buf(),
        source,
    })
}

fn create_directory(path: &Path) -> Result<(), TaskLayoutError> {
    fs::create_dir(path).map_err(|source| {
        if source.kind() == io::ErrorKind::AlreadyExists {
            TaskLayoutError::PathExists {
                path: path.to_path_buf(),
            }
        } else {
            TaskLayoutError::CreateDirectory {
                path: path.to_path_buf(),
                source,
            }
        }
    })
}

fn write_source(path: &Path) -> Result<(), TaskLayoutError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|source| {
            if source.kind() == io::ErrorKind::AlreadyExists {
                TaskLayoutError::PathExists {
                    path: path.to_path_buf(),
                }
            } else {
                TaskLayoutError::WriteSource {
                    path: path.to_path_buf(),
                    source,
                }
            }
        })?;

    file.write_all(DEFAULT_SOURCE.as_bytes())
        .map_err(|source| TaskLayoutError::WriteSource {
            path: path.to_path_buf(),
            source,
        })
}

#[derive(Debug)]
pub enum TaskLayoutError {
    InvalidTaskName { name: String },
    DuplicateTaskName { name: String },
    PathExists { path: PathBuf },
    InspectPath { path: PathBuf, source: io::Error },
    CreateDirectory { path: PathBuf, source: io::Error },
    WriteSource { path: PathBuf, source: io::Error },
}

impl fmt::Display for TaskLayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTaskName { name } => write!(formatter, "invalid task name `{name}`"),
            Self::DuplicateTaskName { name } => write!(formatter, "duplicate task name `{name}`"),
            Self::PathExists { path } => {
                write!(
                    formatter,
                    "task layout path `{}` already exists",
                    path.display()
                )
            }
            Self::InspectPath { path, .. } => {
                write!(
                    formatter,
                    "failed to inspect task layout path `{}`",
                    path.display()
                )
            }
            Self::CreateDirectory { path, .. } => {
                write!(formatter, "failed to create directory `{}`", path.display())
            }
            Self::WriteSource { path, .. } => {
                write!(
                    formatter,
                    "failed to write task source `{}`",
                    path.display()
                )
            }
        }
    }
}

impl Error for TaskLayoutError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InspectPath { source, .. }
            | Self::CreateDirectory { source, .. }
            | Self::WriteSource { source, .. } => Some(source),
            Self::InvalidTaskName { .. }
            | Self::DuplicateTaskName { .. }
            | Self::PathExists { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, process::Command};

    use tempfile::tempdir;

    use super::{generate_task_layout, TaskLayoutError};
    use crate::{
        config::TaskConfig,
        manifest::write_manifest,
        workspace::{create_workspace, WorkspaceRequest},
    };

    fn request(destination: impl Into<std::path::PathBuf>) -> WorkspaceRequest {
        WorkspaceRequest::new(
            destination,
            "abc400",
            vec![
                TaskConfig::new("abc400_a", "a"),
                TaskConfig::new("abc400_b", "b"),
            ],
        )
    }

    #[test]
    fn generates_compilable_sources_and_empty_testcase_directories() {
        let directory = tempdir().expect("temporary directory should be created");
        let request = request(directory.path().join("abc400"));
        let paths = create_workspace(&request).expect("workspace should be created");
        write_manifest(paths.manifest(), request.contest_id(), "2021")
            .expect("manifest should be written");

        generate_task_layout(&request, &paths).expect("task layout should be generated");

        for name in ["a", "b"] {
            let source = paths.source_directory().join(format!("{name}.rs"));
            let contents = fs::read_to_string(source).expect("source should be readable");
            assert!(contents.contains("read_to_string"));

            let testcase_directory = paths.testcase_directory().join(name);
            assert!(testcase_directory.is_dir());
            assert_eq!(
                fs::read_dir(testcase_directory)
                    .expect("testcase directory should be readable")
                    .count(),
                0
            );
        }

        let status = Command::new(env!("CARGO"))
            .args(["check", "--bins", "--offline", "--manifest-path"])
            .arg(paths.manifest())
            .status()
            .expect("cargo check should run");
        assert!(status.success());
    }

    #[test]
    fn rejects_invalid_task_name_before_writing_files() {
        let directory = tempdir().expect("temporary directory should be created");
        let destination = directory.path().join("abc400");
        let request = WorkspaceRequest::new(
            &destination,
            "abc400",
            vec![TaskConfig::new("abc400_a", "../../../escaped")],
        );
        let paths = create_workspace(&request).expect("workspace should be created");

        let error = generate_task_layout(&request, &paths)
            .expect_err("invalid task name should be rejected");

        assert!(matches!(error, TaskLayoutError::InvalidTaskName { .. }));
        assert!(!paths.source_directory().exists());
        assert!(!paths.testcase_directory().exists());
        assert!(!directory.path().join("escaped.rs").exists());
    }

    #[test]
    fn rejects_duplicate_task_names_before_writing_files() {
        let directory = tempdir().expect("temporary directory should be created");
        let destination = directory.path().join("abc400");
        let request = WorkspaceRequest::new(
            &destination,
            "abc400",
            vec![
                TaskConfig::new("abc400_a", "a"),
                TaskConfig::new("abc400_b", "a"),
            ],
        );
        let paths = create_workspace(&request).expect("workspace should be created");

        let error = generate_task_layout(&request, &paths)
            .expect_err("duplicate task name should be rejected");

        assert!(matches!(error, TaskLayoutError::DuplicateTaskName { .. }));
        assert!(!paths.source_directory().exists());
        assert!(!paths.testcase_directory().exists());
    }

    #[test]
    fn does_not_overwrite_existing_layout() {
        let directory = tempdir().expect("temporary directory should be created");
        let request = request(directory.path().join("abc400"));
        let paths = create_workspace(&request).expect("workspace should be created");
        fs::create_dir_all(paths.source_directory()).expect("source directory should be created");
        let existing_source = paths.source_directory().join("a.rs");
        fs::write(&existing_source, "existing").expect("existing source should be written");

        let error =
            generate_task_layout(&request, &paths).expect_err("existing layout should be rejected");

        assert!(matches!(error, TaskLayoutError::PathExists { .. }));
        assert_eq!(
            fs::read_to_string(existing_source).expect("existing source should be readable"),
            "existing"
        );
        assert!(!paths.testcase_directory().exists());
    }
}

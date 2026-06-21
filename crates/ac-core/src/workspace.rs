use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use crate::{config::TaskConfig, manifest::is_valid_package_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRequest {
    destination: PathBuf,
    contest_id: String,
    tasks: Vec<TaskConfig>,
}

impl WorkspaceRequest {
    pub fn new(
        destination: impl Into<PathBuf>,
        contest_id: impl Into<String>,
        tasks: Vec<TaskConfig>,
    ) -> Self {
        Self {
            destination: destination.into(),
            contest_id: contest_id.into(),
            tasks,
        }
    }

    pub fn destination(&self) -> &Path {
        &self.destination
    }

    pub fn contest_id(&self) -> &str {
        &self.contest_id
    }

    pub fn tasks(&self) -> &[TaskConfig] {
        &self.tasks
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePaths {
    root: PathBuf,
    manifest: PathBuf,
    config: PathBuf,
    source_directory: PathBuf,
    testcase_directory: PathBuf,
}

impl WorkspacePaths {
    fn new(root: PathBuf) -> Self {
        Self {
            manifest: root.join("Cargo.toml"),
            config: root.join("ac.toml"),
            source_directory: root.join("src/bin"),
            testcase_directory: root.join("testcases"),
            root,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn manifest(&self) -> &Path {
        &self.manifest
    }

    pub fn config(&self) -> &Path {
        &self.config
    }

    pub fn source_directory(&self) -> &Path {
        &self.source_directory
    }

    pub fn testcase_directory(&self) -> &Path {
        &self.testcase_directory
    }
}

pub fn create_workspace(request: &WorkspaceRequest) -> Result<WorkspacePaths, WorkspaceError> {
    let destination = request.destination();

    if !is_valid_package_name(request.contest_id()) {
        return Err(WorkspaceError::InvalidContestId {
            path: destination.to_path_buf(),
            contest_id: request.contest_id().to_owned(),
        });
    }

    match destination.try_exists() {
        Ok(true) => {
            return Err(WorkspaceError::DestinationExists {
                path: destination.to_path_buf(),
            });
        }
        Ok(false) => {}
        Err(source) => {
            return Err(WorkspaceError::InspectDestination {
                path: destination.to_path_buf(),
                source,
            });
        }
    }

    if let Err(source) = fs::create_dir(destination) {
        if source.kind() == io::ErrorKind::AlreadyExists {
            return Err(WorkspaceError::DestinationExists {
                path: destination.to_path_buf(),
            });
        }

        return Err(WorkspaceError::CreateRoot {
            path: destination.to_path_buf(),
            source,
        });
    }

    Ok(WorkspacePaths::new(destination.to_path_buf()))
}

#[derive(Debug)]
pub enum WorkspaceError {
    InvalidContestId { path: PathBuf, contest_id: String },
    DestinationExists { path: PathBuf },
    InspectDestination { path: PathBuf, source: io::Error },
    CreateRoot { path: PathBuf, source: io::Error },
}

impl WorkspaceError {
    pub fn path(&self) -> &Path {
        match self {
            Self::InvalidContestId { path, .. }
            | Self::DestinationExists { path }
            | Self::InspectDestination { path, .. }
            | Self::CreateRoot { path, .. } => path,
        }
    }
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidContestId { contest_id, .. } => {
                write!(formatter, "invalid contest ID `{contest_id}`")
            }
            Self::DestinationExists { path } => {
                write!(
                    formatter,
                    "workspace destination `{}` already exists",
                    path.display()
                )
            }
            Self::InspectDestination { path, .. } => write!(
                formatter,
                "failed to inspect workspace destination `{}`",
                path.display()
            ),
            Self::CreateRoot { path, .. } => write!(
                formatter,
                "failed to create workspace root `{}`",
                path.display()
            ),
        }
    }
}

impl Error for WorkspaceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidContestId { .. } | Self::DestinationExists { .. } => None,
            Self::InspectDestination { source, .. } | Self::CreateRoot { source, .. } => {
                Some(source)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io};

    use tempfile::tempdir;

    use super::{create_workspace, WorkspaceError, WorkspaceRequest};
    use crate::config::TaskConfig;

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
    fn creates_workspace_root_and_plans_paths() {
        let directory = tempdir().expect("temporary directory should be created");
        let destination = directory.path().join("abc400");
        let request = request(&destination);

        let paths = create_workspace(&request).expect("workspace root should be created");

        assert_eq!(request.contest_id(), "abc400");
        assert_eq!(request.tasks().len(), 2);
        assert!(paths.root().is_dir());
        assert_eq!(paths.root(), destination);
        assert_eq!(paths.manifest(), destination.join("Cargo.toml"));
        assert_eq!(paths.config(), destination.join("ac.toml"));
        assert_eq!(paths.source_directory(), destination.join("src/bin"));
        assert_eq!(paths.testcase_directory(), destination.join("testcases"));
        assert!(!paths.manifest().exists());
        assert!(!paths.source_directory().exists());
        assert!(!paths.testcase_directory().exists());
    }

    #[test]
    fn rejects_existing_destination_without_modifying_it() {
        let directory = tempdir().expect("temporary directory should be created");
        let destination = directory.path().join("abc400");
        fs::create_dir(&destination).expect("destination should be created");
        let marker = destination.join("existing.txt");
        fs::write(&marker, "existing").expect("marker should be written");

        let error = create_workspace(&request(&destination))
            .expect_err("existing destination should be rejected");

        assert_eq!(error.path(), destination);
        assert!(matches!(error, WorkspaceError::DestinationExists { .. }));
        assert_eq!(
            fs::read_to_string(marker).expect("marker should remain readable"),
            "existing"
        );
    }

    #[test]
    fn returns_io_error_when_root_cannot_be_created() {
        let directory = tempdir().expect("temporary directory should be created");
        let destination = directory.path().join("missing-parent/abc400");

        let error = create_workspace(&request(&destination))
            .expect_err("missing parent should prevent creation");

        assert_eq!(error.path(), destination);
        assert!(matches!(
            error,
            WorkspaceError::CreateRoot { source, .. }
                if source.kind() == io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn rejects_invalid_contest_id_before_creating_destination() {
        let directory = tempdir().expect("temporary directory should be created");
        let destination = directory.path().join("outside");
        let request = WorkspaceRequest::new(&destination, "../outside", Vec::new());

        let error = create_workspace(&request).expect_err("invalid contest ID should fail");

        assert_eq!(error.path(), destination);
        assert!(matches!(error, WorkspaceError::InvalidContestId { .. }));
        assert!(!destination.exists());
    }
}

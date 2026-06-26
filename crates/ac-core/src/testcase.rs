use std::{
    error::Error,
    ffi::OsString,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use crate::config::{ProjectConfig, TaskConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredTestcaseFile {
    logical_name: OsString,
    kind: TestcaseFileKind,
    path: PathBuf,
}

impl DiscoveredTestcaseFile {
    fn new(logical_name: OsString, kind: TestcaseFileKind, path: PathBuf) -> Self {
        Self {
            logical_name,
            kind,
            path,
        }
    }

    pub fn logical_name(&self) -> &OsString {
        &self.logical_name
    }

    pub fn kind(&self) -> TestcaseFileKind {
        self.kind
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestcaseFileKind {
    Input,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestcaseDiscovery {
    directory: PathBuf,
    files: Vec<DiscoveredTestcaseFile>,
}

impl TestcaseDiscovery {
    fn new(directory: PathBuf, files: Vec<DiscoveredTestcaseFile>) -> Self {
        Self { directory, files }
    }

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    pub fn files(&self) -> &[DiscoveredTestcaseFile] {
        &self.files
    }
}

pub fn discover_testcase_files(
    project_root: impl AsRef<Path>,
    config: &ProjectConfig,
    task: &TaskConfig,
) -> Result<TestcaseDiscovery, TestcaseDiscoveryError> {
    let directory = project_root
        .as_ref()
        .join(config.testcase_directory())
        .join(task.bin_name());
    let entries =
        fs::read_dir(&directory).map_err(|source| TestcaseDiscoveryError::ReadDirectory {
            path: directory.clone(),
            source,
        })?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|source| TestcaseDiscoveryError::ReadDirectoryEntry {
            path: directory.clone(),
            source,
        })?;
        let path = entry.path();
        let metadata =
            fs::metadata(&path).map_err(|source| TestcaseDiscoveryError::ReadMetadata {
                path: path.clone(),
                source,
            })?;

        if !metadata.is_file() {
            continue;
        }

        let kind = match path.extension().and_then(|extension| extension.to_str()) {
            Some("in") => TestcaseFileKind::Input,
            Some("out") => TestcaseFileKind::Output,
            _ => continue,
        };
        let Some(logical_name) = path.file_stem() else {
            continue;
        };

        files.push(DiscoveredTestcaseFile::new(
            logical_name.to_os_string(),
            kind,
            path,
        ));
    }

    files.sort_by(|left, right| {
        left.logical_name
            .cmp(&right.logical_name)
            .then(left.kind.cmp(&right.kind))
            .then(left.path.cmp(&right.path))
    });

    Ok(TestcaseDiscovery::new(directory, files))
}

#[derive(Debug)]
pub enum TestcaseDiscoveryError {
    ReadDirectory { path: PathBuf, source: io::Error },
    ReadDirectoryEntry { path: PathBuf, source: io::Error },
    ReadMetadata { path: PathBuf, source: io::Error },
}

impl TestcaseDiscoveryError {
    pub fn path(&self) -> &Path {
        match self {
            Self::ReadDirectory { path, .. }
            | Self::ReadDirectoryEntry { path, .. }
            | Self::ReadMetadata { path, .. } => path,
        }
    }
}

impl fmt::Display for TestcaseDiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadDirectory { path, .. } => {
                write!(
                    formatter,
                    "failed to read testcase directory `{}`",
                    path.display()
                )
            }
            Self::ReadDirectoryEntry { path, .. } => write!(
                formatter,
                "failed to read testcase directory entry in `{}`",
                path.display()
            ),
            Self::ReadMetadata { path, .. } => {
                write!(
                    formatter,
                    "failed to read testcase file metadata `{}`",
                    path.display()
                )
            }
        }
    }
}

impl Error for TestcaseDiscoveryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadDirectory { source, .. }
            | Self::ReadDirectoryEntry { source, .. }
            | Self::ReadMetadata { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, fs, io};

    use tempfile::tempdir;

    use super::{discover_testcase_files, TestcaseDiscoveryError, TestcaseFileKind};
    use crate::config::{ProjectConfig, TaskConfig};

    fn config() -> ProjectConfig {
        ProjectConfig::new(
            "abc400",
            "src/bin",
            "testcases",
            "rust",
            "2021",
            vec![TaskConfig::new("abc400_a", "a")],
        )
    }

    fn task() -> TaskConfig {
        TaskConfig::new("abc400_a", "a")
    }

    #[test]
    fn discovers_flat_testcase_files_in_deterministic_logical_name_order() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        fs::write(testcase_directory.join("sample2.out"), "4\n").expect("output should be written");
        fs::write(testcase_directory.join("sample1.out"), "2\n").expect("output should be written");
        fs::write(testcase_directory.join("sample2.in"), "2 2\n").expect("input should be written");
        fs::write(testcase_directory.join("sample1.in"), "1 1\n").expect("input should be written");

        let discovery = discover_testcase_files(directory.path(), &config(), &task())
            .expect("testcases should be discovered");

        assert_eq!(discovery.directory(), testcase_directory);
        let actual: Vec<_> = discovery
            .files()
            .iter()
            .map(|file| {
                (
                    file.logical_name().as_os_str(),
                    file.kind(),
                    file.path()
                        .file_name()
                        .expect("path should have a file name"),
                )
            })
            .collect();
        assert_eq!(
            actual,
            vec![
                (
                    OsStr::new("sample1"),
                    TestcaseFileKind::Input,
                    OsStr::new("sample1.in"),
                ),
                (
                    OsStr::new("sample1"),
                    TestcaseFileKind::Output,
                    OsStr::new("sample1.out"),
                ),
                (
                    OsStr::new("sample2"),
                    TestcaseFileKind::Input,
                    OsStr::new("sample2.in"),
                ),
                (
                    OsStr::new("sample2"),
                    TestcaseFileKind::Output,
                    OsStr::new("sample2.out"),
                ),
            ]
        );
    }

    #[test]
    fn ignores_subdirectories_and_unsupported_extensions() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(testcase_directory.join("nested.in"))
            .expect("nested directory should be created");
        fs::write(testcase_directory.join("sample.in"), "1\n").expect("input should be written");
        fs::write(testcase_directory.join("sample.txt"), "ignored")
            .expect("unsupported file should be written");
        fs::write(testcase_directory.join("README"), "ignored")
            .expect("extensionless file should be written");

        let discovery = discover_testcase_files(directory.path(), &config(), &task())
            .expect("testcases should be discovered");

        assert_eq!(discovery.files().len(), 1);
        assert_eq!(discovery.files()[0].logical_name(), OsStr::new("sample"));
        assert_eq!(discovery.files()[0].kind(), TestcaseFileKind::Input);
    }

    #[test]
    fn keeps_unpaired_candidates_for_later_validation() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        fs::write(testcase_directory.join("input_only.in"), "1\n")
            .expect("input should be written");
        fs::write(testcase_directory.join("output_only.out"), "2\n")
            .expect("output should be written");

        let discovery = discover_testcase_files(directory.path(), &config(), &task())
            .expect("testcases should be discovered");

        let actual: Vec<_> = discovery
            .files()
            .iter()
            .map(|file| (file.logical_name().as_os_str(), file.kind()))
            .collect();
        assert_eq!(
            actual,
            vec![
                (OsStr::new("input_only"), TestcaseFileKind::Input),
                (OsStr::new("output_only"), TestcaseFileKind::Output),
            ]
        );
    }

    #[test]
    fn returns_path_error_when_testcase_directory_is_missing() {
        let directory = tempdir().expect("temporary directory should be created");
        let expected = directory.path().join("testcases/a");

        let error = discover_testcase_files(directory.path(), &config(), &task())
            .expect_err("missing directory should fail");

        assert_eq!(error.path(), expected);
        assert!(matches!(
            &error,
            TestcaseDiscoveryError::ReadDirectory { source, .. }
                if source.kind() == io::ErrorKind::NotFound
        ));
        assert!(error.to_string().contains(&expected.display().to_string()));
    }

    #[cfg(unix)]
    #[test]
    fn returns_path_error_when_file_metadata_cannot_be_read() {
        use std::os::unix::fs::symlink;

        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        let broken_link = testcase_directory.join("broken.in");
        symlink(testcase_directory.join("missing"), &broken_link)
            .expect("broken symlink should be created");

        let error = discover_testcase_files(directory.path(), &config(), &task())
            .expect_err("broken symlink metadata should fail");

        assert_eq!(error.path(), broken_link);
        assert!(matches!(
            &error,
            TestcaseDiscoveryError::ReadMetadata { source, .. }
                if source.kind() == io::ErrorKind::NotFound
        ));
    }
}

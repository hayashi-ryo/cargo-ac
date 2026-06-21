use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProjectConfig {
    contest_id: String,
    source_directory: PathBuf,
    testcase_directory: PathBuf,
    language: String,
    rust_edition: String,
    tasks: Vec<TaskConfig>,
}

impl ProjectConfig {
    pub fn new(
        contest_id: impl Into<String>,
        source_directory: impl Into<PathBuf>,
        testcase_directory: impl Into<PathBuf>,
        language: impl Into<String>,
        rust_edition: impl Into<String>,
        tasks: Vec<TaskConfig>,
    ) -> Self {
        Self {
            contest_id: contest_id.into(),
            source_directory: source_directory.into(),
            testcase_directory: testcase_directory.into(),
            language: language.into(),
            rust_edition: rust_edition.into(),
            tasks,
        }
    }

    pub fn contest_id(&self) -> &str {
        &self.contest_id
    }

    pub fn source_directory(&self) -> &Path {
        &self.source_directory
    }

    pub fn testcase_directory(&self) -> &Path {
        &self.testcase_directory
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn rust_edition(&self) -> &str {
        &self.rust_edition
    }

    pub fn tasks(&self) -> &[TaskConfig] {
        &self.tasks
    }

    pub fn read(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;

        toml::from_str(&contents).map_err(|_| ConfigError::Parse {
            path: path.to_path_buf(),
        })
    }

    pub fn write(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let path = path.as_ref();
        let contents = toml::to_string_pretty(self).map_err(|source| ConfigError::Serialize {
            path: path.to_path_buf(),
            source,
        })?;

        fs::write(path, contents).map_err(|source| ConfigError::Write {
            path: path.to_path_buf(),
            source,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TaskConfig {
    task_id: String,
    bin_name: String,
}

impl TaskConfig {
    pub fn new(task_id: impl Into<String>, bin_name: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            bin_name: bin_name.into(),
        }
    }

    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    pub fn bin_name(&self) -> &str {
        &self.bin_name
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Read {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
    },
    Serialize {
        path: PathBuf,
        source: toml::ser::Error,
    },
    Write {
        path: PathBuf,
        source: io::Error,
    },
}

impl ConfigError {
    pub fn path(&self) -> &Path {
        match self {
            Self::Read { path, .. }
            | Self::Parse { path }
            | Self::Serialize { path, .. }
            | Self::Write { path, .. } => path,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, .. } => {
                write!(formatter, "failed to read config `{}`", path.display())
            }
            Self::Parse { path } => {
                write!(formatter, "failed to parse config `{}`", path.display())
            }
            Self::Serialize { path, .. } => {
                write!(formatter, "failed to serialize config `{}`", path.display())
            }
            Self::Write { path, .. } => {
                write!(formatter, "failed to write config `{}`", path.display())
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } | Self::Write { source, .. } => Some(source),
            Self::Serialize { source, .. } => Some(source),
            Self::Parse { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io, path::Path};

    use tempfile::tempdir;

    use super::{ConfigError, ProjectConfig, TaskConfig};

    fn project_config() -> ProjectConfig {
        ProjectConfig::new(
            "abc400",
            "src/bin",
            "testcases",
            "rust",
            "2021",
            vec![
                TaskConfig::new("abc400_a", "a"),
                TaskConfig::new("abc400_b", "b"),
            ],
        )
    }

    #[test]
    fn represents_project_configuration() {
        let config = project_config();

        assert_eq!(config.contest_id(), "abc400");
        assert_eq!(config.source_directory(), Path::new("src/bin"));
        assert_eq!(config.testcase_directory(), Path::new("testcases"));
        assert_eq!(config.language(), "rust");
        assert_eq!(config.rust_edition(), "2021");
        assert_eq!(config.tasks().len(), 2);
        assert_eq!(config.tasks()[0].task_id(), "abc400_a");
        assert_eq!(config.tasks()[0].bin_name(), "a");
    }

    #[test]
    fn writes_and_reads_project_configuration() {
        let directory = tempdir().expect("temporary directory should be created");
        let path = directory.path().join("ac.toml");
        let expected = project_config();

        expected.write(&path).expect("config should be written");
        let actual = ProjectConfig::read(&path).expect("config should be read");

        assert_eq!(actual, expected);
    }

    #[test]
    fn returns_read_error_for_missing_file() {
        let directory = tempdir().expect("temporary directory should be created");
        let path = directory.path().join("missing.toml");

        let error = ProjectConfig::read(&path).expect_err("missing config should fail");

        assert_eq!(error.path(), path);
        assert!(matches!(
            error,
            ConfigError::Read { source, .. } if source.kind() == io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn returns_sanitized_parse_error_for_invalid_toml() {
        let directory = tempdir().expect("temporary directory should be created");
        let path = directory.path().join("ac.toml");
        fs::write(&path, "cookie = \"secret-value").expect("invalid config should be written");

        let error = ProjectConfig::read(&path).expect_err("invalid config should fail");
        let message = error.to_string();

        assert_eq!(error.path(), path);
        assert!(matches!(error, ConfigError::Parse { .. }));
        assert!(!message.contains("secret-value"));
    }
}

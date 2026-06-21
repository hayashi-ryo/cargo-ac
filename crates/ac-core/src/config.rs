use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{ProjectConfig, TaskConfig};

    #[test]
    fn represents_project_configuration() {
        let tasks = vec![
            TaskConfig::new("abc400_a", "a"),
            TaskConfig::new("abc400_b", "b"),
        ];

        let config = ProjectConfig::new(
            "abc400",
            "src/bin",
            "testcases",
            "rust",
            "2021",
            tasks.clone(),
        );

        assert_eq!(config.contest_id(), "abc400");
        assert_eq!(config.source_directory(), Path::new("src/bin"));
        assert_eq!(config.testcase_directory(), Path::new("testcases"));
        assert_eq!(config.language(), "rust");
        assert_eq!(config.rust_edition(), "2021");
        assert_eq!(config.tasks(), tasks);
        assert_eq!(config.tasks()[0].task_id(), "abc400_a");
        assert_eq!(config.tasks()[0].bin_name(), "a");
    }
}

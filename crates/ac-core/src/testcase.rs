use std::{
    error::Error,
    ffi::OsString,
    fmt, fs,
    fs::OpenOptions,
    io,
    io::Write,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestcasePair {
    logical_name: OsString,
    input_path: PathBuf,
    output_path: PathBuf,
    input: Vec<u8>,
    expected_output: Vec<u8>,
}

impl TestcasePair {
    fn new(
        logical_name: OsString,
        input_path: PathBuf,
        output_path: PathBuf,
        input: Vec<u8>,
        expected_output: Vec<u8>,
    ) -> Self {
        Self {
            logical_name,
            input_path,
            output_path,
            input,
            expected_output,
        }
    }

    pub fn logical_name(&self) -> &OsString {
        &self.logical_name
    }

    pub fn input_path(&self) -> &Path {
        &self.input_path
    }

    pub fn output_path(&self) -> &Path {
        &self.output_path
    }

    pub fn input(&self) -> &[u8] {
        &self.input
    }

    pub fn expected_output(&self) -> &[u8] {
        &self.expected_output
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddedTestcase {
    logical_name: String,
    input_path: PathBuf,
    output_path: PathBuf,
}

impl AddedTestcase {
    fn new(logical_name: String, input_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            logical_name,
            input_path,
            output_path,
        }
    }

    pub fn logical_name(&self) -> &str {
        &self.logical_name
    }

    pub fn input_path(&self) -> &Path {
        &self.input_path
    }

    pub fn output_path(&self) -> &Path {
        &self.output_path
    }
}

pub fn add_custom_testcase(
    project_root: impl AsRef<Path>,
    config: &ProjectConfig,
    task: &TaskConfig,
    input: &[u8],
    expected_output: &[u8],
) -> Result<AddedTestcase, AddCustomTestcaseError> {
    let discovery = discover_testcase_files(&project_root, config, task)
        .map_err(AddCustomTestcaseError::Discover)?;
    let case_number = next_custom_case_number(&discovery);
    let logical_name = format!("custom-{case_number}");
    let input_path = discovery.directory().join(format!("{logical_name}.in"));
    let output_path = discovery.directory().join(format!("{logical_name}.out"));

    write_new_file(&input_path, input).map_err(|source| AddCustomTestcaseError::WriteInput {
        path: input_path.clone(),
        source,
    })?;
    write_new_file(&output_path, expected_output).map_err(|source| {
        AddCustomTestcaseError::WriteOutput {
            path: output_path.clone(),
            source,
        }
    })?;

    Ok(AddedTestcase::new(logical_name, input_path, output_path))
}

fn next_custom_case_number(discovery: &TestcaseDiscovery) -> usize {
    let mut number = 1;

    loop {
        let logical_name = format!("custom-{number}");
        if discovery
            .files()
            .iter()
            .all(|file| file.logical_name().to_string_lossy() != logical_name)
        {
            return number;
        }

        number += 1;
    }
}

fn write_new_file(path: &Path, contents: &[u8]) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(contents)
}

pub fn validate_testcase_pairs(
    discovery: &TestcaseDiscovery,
) -> Result<Vec<TestcasePair>, TestcaseValidationError> {
    validate_testcase_file_candidates(discovery.files())
}

fn validate_testcase_file_candidates(
    files: &[DiscoveredTestcaseFile],
) -> Result<Vec<TestcasePair>, TestcaseValidationError> {
    let candidates = collect_pair_candidates(files);
    let problems = collect_pair_problems(&candidates);

    if !problems.is_empty() {
        return Err(TestcaseValidationError::InvalidPairs { problems });
    }

    candidates
        .into_iter()
        .map(|candidate| {
            let input_path = single_path(
                candidate.logical_name.clone(),
                TestcaseFileKind::Input,
                candidate.input_paths,
            )?;
            let output_path = single_path(
                candidate.logical_name.clone(),
                TestcaseFileKind::Output,
                candidate.output_paths,
            )?;
            let input =
                fs::read(&input_path).map_err(|source| TestcaseValidationError::ReadFile {
                    path: input_path.clone(),
                    source,
                })?;
            let expected_output =
                fs::read(&output_path).map_err(|source| TestcaseValidationError::ReadFile {
                    path: output_path.clone(),
                    source,
                })?;

            Ok(TestcasePair::new(
                candidate.logical_name,
                input_path,
                output_path,
                input,
                expected_output,
            ))
        })
        .collect()
}

fn single_path(
    logical_name: OsString,
    kind: TestcaseFileKind,
    paths: Vec<PathBuf>,
) -> Result<PathBuf, TestcaseValidationError> {
    match (kind, paths.as_slice()) {
        (TestcaseFileKind::Input, []) => Err(TestcaseValidationError::InvalidPairs {
            problems: vec![TestcasePairProblem::MissingInput { logical_name }],
        }),
        (TestcaseFileKind::Output, []) => Err(TestcaseValidationError::InvalidPairs {
            problems: vec![TestcasePairProblem::MissingOutput { logical_name }],
        }),
        (_, [path]) => Ok(path.clone()),
        (TestcaseFileKind::Input, _) => Err(TestcaseValidationError::InvalidPairs {
            problems: vec![TestcasePairProblem::DuplicateInput {
                logical_name,
                paths,
            }],
        }),
        (TestcaseFileKind::Output, _) => Err(TestcaseValidationError::InvalidPairs {
            problems: vec![TestcasePairProblem::DuplicateOutput {
                logical_name,
                paths,
            }],
        }),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PairCandidate {
    logical_name: OsString,
    input_paths: Vec<PathBuf>,
    output_paths: Vec<PathBuf>,
}

impl PairCandidate {
    fn new(logical_name: OsString) -> Self {
        Self {
            logical_name,
            input_paths: Vec::new(),
            output_paths: Vec::new(),
        }
    }
}

fn collect_pair_candidates(files: &[DiscoveredTestcaseFile]) -> Vec<PairCandidate> {
    let mut candidates: Vec<PairCandidate> = Vec::new();

    for file in files {
        let candidate = match candidates
            .iter_mut()
            .find(|candidate| candidate.logical_name == file.logical_name)
        {
            Some(candidate) => candidate,
            None => {
                candidates.push(PairCandidate::new(file.logical_name.clone()));
                candidates.last_mut().expect("candidate was just pushed")
            }
        };

        match file.kind {
            TestcaseFileKind::Input => candidate.input_paths.push(file.path.clone()),
            TestcaseFileKind::Output => candidate.output_paths.push(file.path.clone()),
        }
    }

    candidates
}

fn collect_pair_problems(candidates: &[PairCandidate]) -> Vec<TestcasePairProblem> {
    let mut problems = Vec::new();

    for candidate in candidates {
        match candidate.input_paths.len() {
            0 => problems.push(TestcasePairProblem::MissingInput {
                logical_name: candidate.logical_name.clone(),
            }),
            1 => {}
            _ => problems.push(TestcasePairProblem::DuplicateInput {
                logical_name: candidate.logical_name.clone(),
                paths: candidate.input_paths.clone(),
            }),
        }

        match candidate.output_paths.len() {
            0 => problems.push(TestcasePairProblem::MissingOutput {
                logical_name: candidate.logical_name.clone(),
            }),
            1 => {}
            _ => problems.push(TestcasePairProblem::DuplicateOutput {
                logical_name: candidate.logical_name.clone(),
                paths: candidate.output_paths.clone(),
            }),
        }
    }

    problems
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

#[derive(Debug)]
pub enum AddCustomTestcaseError {
    Discover(TestcaseDiscoveryError),
    WriteInput { path: PathBuf, source: io::Error },
    WriteOutput { path: PathBuf, source: io::Error },
}

impl AddCustomTestcaseError {
    pub fn path(&self) -> &Path {
        match self {
            Self::Discover(source) => source.path(),
            Self::WriteInput { path, .. } | Self::WriteOutput { path, .. } => path,
        }
    }
}

impl fmt::Display for AddCustomTestcaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discover(source) => write!(formatter, "{source}"),
            Self::WriteInput { path, .. } => {
                write!(
                    formatter,
                    "failed to write testcase input `{}`",
                    path.display()
                )
            }
            Self::WriteOutput { path, .. } => write!(
                formatter,
                "failed to write testcase output `{}`",
                path.display()
            ),
        }
    }
}

impl Error for AddCustomTestcaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Discover(source) => Some(source),
            Self::WriteInput { source, .. } | Self::WriteOutput { source, .. } => Some(source),
        }
    }
}

#[derive(Debug)]
pub enum TestcaseValidationError {
    InvalidPairs { problems: Vec<TestcasePairProblem> },
    ReadFile { path: PathBuf, source: io::Error },
}

impl TestcaseValidationError {
    pub fn problems(&self) -> &[TestcasePairProblem] {
        match self {
            Self::InvalidPairs { problems } => problems,
            Self::ReadFile { .. } => &[],
        }
    }

    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::InvalidPairs { .. } => None,
            Self::ReadFile { path, .. } => Some(path),
        }
    }
}

impl fmt::Display for TestcaseValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPairs { problems } => {
                write!(
                    formatter,
                    "found {} invalid testcase pair candidate(s)",
                    problems.len()
                )?;

                for problem in problems {
                    write!(formatter, ": {problem}")?;
                }

                Ok(())
            }
            Self::ReadFile { path, .. } => {
                write!(
                    formatter,
                    "failed to read testcase file `{}`",
                    path.display()
                )
            }
        }
    }
}

impl Error for TestcaseValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPairs { .. } => None,
            Self::ReadFile { source, .. } => Some(source),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestcasePairProblem {
    MissingInput {
        logical_name: OsString,
    },
    MissingOutput {
        logical_name: OsString,
    },
    DuplicateInput {
        logical_name: OsString,
        paths: Vec<PathBuf>,
    },
    DuplicateOutput {
        logical_name: OsString,
        paths: Vec<PathBuf>,
    },
}

impl TestcasePairProblem {
    pub fn logical_name(&self) -> &OsString {
        match self {
            Self::MissingInput { logical_name }
            | Self::MissingOutput { logical_name }
            | Self::DuplicateInput { logical_name, .. }
            | Self::DuplicateOutput { logical_name, .. } => logical_name,
        }
    }

    pub fn kind(&self) -> TestcaseFileKind {
        match self {
            Self::MissingInput { .. } | Self::DuplicateInput { .. } => TestcaseFileKind::Input,
            Self::MissingOutput { .. } | Self::DuplicateOutput { .. } => TestcaseFileKind::Output,
        }
    }

    pub fn paths(&self) -> &[PathBuf] {
        match self {
            Self::DuplicateInput { paths, .. } | Self::DuplicateOutput { paths, .. } => paths,
            Self::MissingInput { .. } | Self::MissingOutput { .. } => &[],
        }
    }
}

impl fmt::Display for TestcasePairProblem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput { logical_name } => write!(
                formatter,
                "missing input for `{}`",
                logical_name.to_string_lossy()
            ),
            Self::MissingOutput { logical_name } => write!(
                formatter,
                "missing output for `{}`",
                logical_name.to_string_lossy()
            ),
            Self::DuplicateInput {
                logical_name,
                paths,
            } => write!(
                formatter,
                "duplicate input for `{}` ({} candidate(s))",
                logical_name.to_string_lossy(),
                paths.len()
            ),
            Self::DuplicateOutput {
                logical_name,
                paths,
            } => write!(
                formatter,
                "duplicate output for `{}` ({} candidate(s))",
                logical_name.to_string_lossy(),
                paths.len()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::{OsStr, OsString},
        fs, io,
        path::PathBuf,
    };

    use tempfile::tempdir;

    use super::{
        add_custom_testcase, discover_testcase_files, validate_testcase_file_candidates,
        validate_testcase_pairs, AddCustomTestcaseError, DiscoveredTestcaseFile,
        TestcaseDiscoveryError, TestcaseFileKind, TestcasePairProblem, TestcaseValidationError,
    };
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

    fn candidate(
        logical_name: impl Into<OsString>,
        kind: TestcaseFileKind,
        path: impl Into<PathBuf>,
    ) -> DiscoveredTestcaseFile {
        DiscoveredTestcaseFile::new(logical_name.into(), kind, path.into())
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

    #[test]
    fn adds_custom_testcase_with_next_available_number() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        fs::write(testcase_directory.join("custom-1.in"), "existing input\n")
            .expect("existing input should be written");
        fs::write(testcase_directory.join("custom-2.out"), "existing output\n")
            .expect("existing output should be written");

        let added = add_custom_testcase(
            directory.path(),
            &config(),
            &task(),
            b"new input\n",
            b"new output\n",
        )
        .expect("custom testcase should be added");

        assert_eq!(added.logical_name(), "custom-3");
        assert_eq!(added.input_path(), testcase_directory.join("custom-3.in"));
        assert_eq!(added.output_path(), testcase_directory.join("custom-3.out"));
        assert_eq!(
            fs::read(testcase_directory.join("custom-3.in")).expect("input should be readable"),
            b"new input\n"
        );
        assert_eq!(
            fs::read(testcase_directory.join("custom-3.out")).expect("output should be readable"),
            b"new output\n"
        );
        assert_eq!(
            fs::read_to_string(testcase_directory.join("custom-1.in"))
                .expect("existing input should remain"),
            "existing input\n"
        );
    }

    #[test]
    fn add_custom_testcase_returns_directory_read_error() {
        let directory = tempdir().expect("temporary directory should be created");
        let expected = directory.path().join("testcases/a");

        let error = add_custom_testcase(directory.path(), &config(), &task(), b"in", b"out")
            .expect_err("missing directory should fail");

        assert_eq!(error.path(), expected);
        assert!(matches!(
            error,
            AddCustomTestcaseError::Discover(TestcaseDiscoveryError::ReadDirectory { .. })
        ));
    }

    #[test]
    fn add_custom_testcase_does_not_overwrite_existing_candidate() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        fs::write(testcase_directory.join("custom-1.in"), "existing\n")
            .expect("existing input should be written");
        fs::write(testcase_directory.join("custom-1.out"), "existing\n")
            .expect("existing output should be written");

        let added = add_custom_testcase(directory.path(), &config(), &task(), b"in\n", b"out\n")
            .expect("custom testcase should be added");

        assert_eq!(added.logical_name(), "custom-2");
        assert_eq!(
            fs::read_to_string(testcase_directory.join("custom-1.in"))
                .expect("existing input should remain"),
            "existing\n"
        );
    }

    #[test]
    fn add_custom_testcase_reports_output_write_failure_after_input_creation() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(testcase_directory.join("custom-1.out"))
            .expect("conflicting output directory should be created");

        let error = add_custom_testcase(directory.path(), &config(), &task(), b"in\n", b"out\n")
            .expect_err("output path conflict should fail");

        assert_eq!(error.path(), testcase_directory.join("custom-1.out"));
        assert!(matches!(error, AddCustomTestcaseError::WriteOutput { .. }));
        assert_eq!(
            fs::read_to_string(testcase_directory.join("custom-1.in"))
                .expect("input side should have been written before output failure"),
            "in\n"
        );
        assert!(testcase_directory.join("custom-1.out").is_dir());
    }

    #[test]
    fn validates_pairs_and_reads_contents_as_bytes() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        fs::write(testcase_directory.join("binary.in"), [0xff, 0x00, b'\n'])
            .expect("input should be written");
        fs::write(testcase_directory.join("binary.out"), [0xfe, 0x01, b'\n'])
            .expect("output should be written");

        let discovery = discover_testcase_files(directory.path(), &config(), &task())
            .expect("testcases should be discovered");
        let pairs = validate_testcase_pairs(&discovery).expect("pairs should be valid");

        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].logical_name(), OsStr::new("binary"));
        assert_eq!(pairs[0].input_path(), testcase_directory.join("binary.in"));
        assert_eq!(
            pairs[0].output_path(),
            testcase_directory.join("binary.out")
        );
        assert_eq!(pairs[0].input(), &[0xff, 0x00, b'\n']);
        assert_eq!(pairs[0].expected_output(), &[0xfe, 0x01, b'\n']);
    }

    #[test]
    fn keeps_valid_pair_order_from_discovery() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        fs::write(testcase_directory.join("sample1.in"), "1\n").expect("input should be written");
        fs::write(testcase_directory.join("sample1.out"), "1\n").expect("output should be written");
        fs::write(testcase_directory.join("sample2.in"), "2\n").expect("input should be written");
        fs::write(testcase_directory.join("sample2.out"), "2\n").expect("output should be written");

        let discovery = discover_testcase_files(directory.path(), &config(), &task())
            .expect("testcases should be discovered");
        let pairs = validate_testcase_pairs(&discovery).expect("pairs should be valid");
        let actual: Vec<_> = pairs
            .iter()
            .map(|pair| pair.logical_name().as_os_str())
            .collect();

        assert_eq!(actual, vec![OsStr::new("sample1"), OsStr::new("sample2")]);
    }

    #[test]
    fn reports_missing_input_and_output_with_case_names() {
        let files = vec![
            candidate("input_only", TestcaseFileKind::Input, "input_only.in"),
            candidate("output_only", TestcaseFileKind::Output, "output_only.out"),
        ];

        let error = validate_testcase_file_candidates(&files)
            .expect_err("missing pair sides should fail validation");

        assert!(matches!(
            &error,
            TestcaseValidationError::InvalidPairs { .. }
        ));
        assert_eq!(
            error.problems(),
            &[
                TestcasePairProblem::MissingOutput {
                    logical_name: "input_only".into(),
                },
                TestcasePairProblem::MissingInput {
                    logical_name: "output_only".into(),
                },
            ]
        );
        let message = error.to_string();
        assert!(message.contains("input_only"));
        assert!(message.contains("missing output"));
        assert!(message.contains("output_only"));
        assert!(message.contains("missing input"));
    }

    #[test]
    fn reports_duplicate_input_and_output_without_choosing_one() {
        let files = vec![
            candidate("sample", TestcaseFileKind::Input, "sample.in"),
            candidate("sample", TestcaseFileKind::Input, "sample.copy.in"),
            candidate("sample", TestcaseFileKind::Output, "sample.out"),
            candidate("sample2", TestcaseFileKind::Input, "sample2.in"),
            candidate("sample2", TestcaseFileKind::Output, "sample2.out"),
            candidate("sample2", TestcaseFileKind::Output, "sample2.copy.out"),
        ];

        let error = validate_testcase_file_candidates(&files)
            .expect_err("duplicate pair sides should fail validation");

        assert_eq!(
            error.problems(),
            &[
                TestcasePairProblem::DuplicateInput {
                    logical_name: "sample".into(),
                    paths: vec!["sample.in".into(), "sample.copy.in".into()],
                },
                TestcasePairProblem::DuplicateOutput {
                    logical_name: "sample2".into(),
                    paths: vec!["sample2.out".into(), "sample2.copy.out".into()],
                },
            ]
        );
        let message = error.to_string();
        assert!(message.contains("sample"));
        assert!(message.contains("duplicate input"));
        assert!(message.contains("sample2"));
        assert!(message.contains("duplicate output"));
    }

    #[test]
    fn returns_file_path_when_pair_content_cannot_be_read() {
        let directory = tempdir().expect("temporary directory should be created");
        let testcase_directory = directory.path().join("testcases/a");
        fs::create_dir_all(&testcase_directory).expect("testcase directory should be created");
        let input_path = testcase_directory.join("sample.in");
        let output_path = testcase_directory.join("sample.out");
        fs::write(&input_path, "1\n").expect("input should be written");
        fs::write(&output_path, "1\n").expect("output should be written");
        let discovery = discover_testcase_files(directory.path(), &config(), &task())
            .expect("testcases should be discovered");
        fs::remove_file(&output_path).expect("output should be removed");

        let error = validate_testcase_pairs(&discovery).expect_err("missing file read should fail");

        assert_eq!(error.path(), Some(output_path.as_path()));
        assert!(matches!(
            &error,
            TestcaseValidationError::ReadFile { source, .. }
                if source.kind() == io::ErrorKind::NotFound
        ));
        assert!(error
            .to_string()
            .contains(&output_path.display().to_string()));
    }
}

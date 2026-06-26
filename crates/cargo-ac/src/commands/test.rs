use ac_core::{
    config::{ProjectConfig, TaskConfig},
    output::{compare_output, WrongAnswerDiff, WrongAnswerDiffLine},
    runner::{run_task_binary, RunnerProfile, TaskExecution, TaskRunRequest},
    testcase::{discover_testcase_files, validate_testcase_pairs, TestcasePair},
};

use crate::error::CliResult;

use std::{error::Error, fmt, io, io::Write, path::Path, process::Command, time::Duration};

const CONFIG_FILE: &str = "ac.toml";
const ALL_SELECTOR: &str = "all";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

pub(crate) fn run(task: String, release: bool) -> CliResult {
    let current_directory = std::env::current_dir()?;
    let failed = run_in_workspace(&mut io::stdout(), &current_directory, &task, release)?;

    if failed {
        return Err(Box::new(TestCommandError::Failed));
    }

    Ok(())
}

fn run_in_workspace(
    writer: &mut impl Write,
    workspace_root: &Path,
    task_selector: &str,
    release: bool,
) -> Result<bool, Box<dyn Error>> {
    let config = ProjectConfig::read(workspace_root.join(CONFIG_FILE))?;
    let tasks = select_tasks(&config, task_selector)?;
    let profile = if release {
        RunnerProfile::Release
    } else {
        RunnerProfile::Debug
    };
    let mut failed = false;

    for task in tasks {
        build_task(workspace_root, task.bin_name(), profile)?;
        let result = run_task_cases(workspace_root, &config, task, profile)?;
        write_task_result(writer, &result)?;

        let summary = result.summary();
        if summary.total() == 0
            || summary.wrong_answer() > 0
            || summary.runtime_error() > 0
            || summary.time_limit_exceeded() > 0
        {
            failed = true;
        }
    }

    Ok(failed)
}

fn select_tasks<'a>(
    config: &'a ProjectConfig,
    task_selector: &str,
) -> Result<Vec<&'a TaskConfig>, TestCommandError> {
    if task_selector == ALL_SELECTOR {
        return Ok(config.tasks().iter().collect());
    }

    config
        .tasks()
        .iter()
        .find(|task| task.bin_name() == task_selector)
        .map(|task| vec![task])
        .ok_or_else(|| TestCommandError::UnknownTask {
            task: task_selector.to_owned(),
            available: config
                .tasks()
                .iter()
                .map(|task| task.bin_name().to_owned())
                .collect(),
        })
}

fn build_task(
    workspace_root: &Path,
    task_name: &str,
    profile: RunnerProfile,
) -> Result<(), TestCommandError> {
    let mut command = Command::new(env!("CARGO"));
    command
        .current_dir(workspace_root)
        .arg("build")
        .arg("--quiet")
        .arg("--bin")
        .arg(task_name);

    if profile == RunnerProfile::Release {
        command.arg("--release");
    }

    let output = command
        .output()
        .map_err(|source| TestCommandError::BuildSpawn {
            task: task_name.to_owned(),
            source,
        })?;

    if !output.status.success() {
        return Err(TestCommandError::BuildFailed {
            task: task_name.to_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    Ok(())
}

fn run_task_cases(
    workspace_root: &Path,
    config: &ProjectConfig,
    task: &TaskConfig,
    profile: RunnerProfile,
) -> Result<TaskDisplayResult, Box<dyn Error>> {
    let discovery = discover_testcase_files(workspace_root, config, task)?;
    let pairs = validate_testcase_pairs(&discovery)?;
    let cases = pairs
        .iter()
        .map(|pair| run_testcase_pair(workspace_root, task.bin_name(), pair, profile))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TaskDisplayResult::new(task.bin_name(), cases))
}

fn run_testcase_pair(
    workspace_root: &Path,
    task_name: &str,
    pair: &TestcasePair,
    profile: RunnerProfile,
) -> Result<CaseDisplayResult, Box<dyn Error>> {
    let request = TaskRunRequest::new(workspace_root, task_name, profile, DEFAULT_TIMEOUT);
    let execution = run_task_binary(&request, pair.input())?;
    let case_name = pair.logical_name().to_string_lossy().into_owned();

    let outcome = match execution {
        TaskExecution::TimedOut(_) => CaseOutcome::TimeLimitExceeded,
        TaskExecution::Finished(output) if !output.exit_status().success() => {
            CaseOutcome::RuntimeError {
                exit_status: format_exit_status(output.exit_status().code()),
                stderr: String::from_utf8_lossy(output.stderr()).into_owned(),
            }
        }
        TaskExecution::Finished(output) => {
            let comparison = compare_output(pair.expected_output(), output.stdout())?;

            if comparison.matches() {
                CaseOutcome::Accepted
            } else {
                CaseOutcome::WrongAnswer {
                    diff: comparison.wrong_answer_diff(),
                }
            }
        }
    };

    Ok(CaseDisplayResult::new(case_name, outcome))
}

fn format_exit_status(code: Option<i32>) -> String {
    match code {
        Some(code) => format!("exit code {code}"),
        None => "terminated by signal".to_owned(),
    }
}

#[allow(dead_code)]
pub(crate) fn write_task_result(
    writer: &mut impl Write,
    result: &TaskDisplayResult,
) -> io::Result<()> {
    writeln!(writer, "task {}", result.task_name())?;

    for case in result.cases() {
        match case.outcome() {
            CaseOutcome::Accepted => {
                writeln!(writer, "[AC] {}", case.case_name())?;
            }
            CaseOutcome::WrongAnswer { diff } => {
                writeln!(writer, "[WA] {}", case.case_name())?;
                if let Some(diff) = diff {
                    write!(writer, "{}", format_wrong_answer_diff(diff))?;
                }
            }
            CaseOutcome::RuntimeError {
                exit_status,
                stderr,
            } => {
                writeln!(writer, "[RE] {}", case.case_name())?;
                writeln!(writer, "  exit status: {exit_status}")?;

                if !stderr.is_empty() {
                    writeln!(writer, "  stderr:")?;
                    for line in stderr.lines() {
                        writeln!(writer, "    {line}")?;
                    }
                }
            }
            CaseOutcome::TimeLimitExceeded => {
                writeln!(writer, "[TLE] {} timed out", case.case_name())?;
            }
        }
    }

    let summary = result.summary();
    writeln!(
        writer,
        "summary: AC {} WA {} RE {} TLE {} total {}",
        summary.accepted(),
        summary.wrong_answer(),
        summary.runtime_error(),
        summary.time_limit_exceeded(),
        summary.total()
    )
}

#[allow(dead_code)]
pub(crate) fn format_task_result(result: &TaskDisplayResult) -> io::Result<String> {
    let mut output = Vec::new();
    write_task_result(&mut output, result)?;

    Ok(String::from_utf8(output).expect("formatter writes UTF-8 text"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TaskDisplayResult {
    task_name: String,
    cases: Vec<CaseDisplayResult>,
}

#[allow(dead_code)]
impl TaskDisplayResult {
    pub(crate) fn new(task_name: impl Into<String>, cases: Vec<CaseDisplayResult>) -> Self {
        Self {
            task_name: task_name.into(),
            cases,
        }
    }

    pub(crate) fn task_name(&self) -> &str {
        &self.task_name
    }

    pub(crate) fn cases(&self) -> &[CaseDisplayResult] {
        &self.cases
    }

    pub(crate) fn summary(&self) -> TaskResultSummary {
        TaskResultSummary::from_cases(&self.cases)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CaseDisplayResult {
    case_name: String,
    outcome: CaseOutcome,
}

#[allow(dead_code)]
impl CaseDisplayResult {
    pub(crate) fn new(case_name: impl Into<String>, outcome: CaseOutcome) -> Self {
        Self {
            case_name: case_name.into(),
            outcome,
        }
    }

    pub(crate) fn case_name(&self) -> &str {
        &self.case_name
    }

    pub(crate) fn outcome(&self) -> &CaseOutcome {
        &self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum CaseOutcome {
    Accepted,
    WrongAnswer { diff: Option<WrongAnswerDiff> },
    RuntimeError { exit_status: String, stderr: String },
    TimeLimitExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TaskResultSummary {
    accepted: usize,
    wrong_answer: usize,
    runtime_error: usize,
    time_limit_exceeded: usize,
}

impl TaskResultSummary {
    fn from_cases(cases: &[CaseDisplayResult]) -> Self {
        let mut summary = Self {
            accepted: 0,
            wrong_answer: 0,
            runtime_error: 0,
            time_limit_exceeded: 0,
        };

        for case in cases {
            match case.outcome() {
                CaseOutcome::Accepted => summary.accepted += 1,
                CaseOutcome::WrongAnswer { .. } => summary.wrong_answer += 1,
                CaseOutcome::RuntimeError { .. } => summary.runtime_error += 1,
                CaseOutcome::TimeLimitExceeded => summary.time_limit_exceeded += 1,
            }
        }

        summary
    }

    pub(crate) fn accepted(self) -> usize {
        self.accepted
    }

    pub(crate) fn wrong_answer(self) -> usize {
        self.wrong_answer
    }

    pub(crate) fn runtime_error(self) -> usize {
        self.runtime_error
    }

    pub(crate) fn time_limit_exceeded(self) -> usize {
        self.time_limit_exceeded
    }

    pub(crate) fn total(self) -> usize {
        self.accepted + self.wrong_answer + self.runtime_error + self.time_limit_exceeded
    }
}

#[derive(Debug)]
enum TestCommandError {
    UnknownTask {
        task: String,
        available: Vec<String>,
    },
    BuildSpawn {
        task: String,
        source: io::Error,
    },
    BuildFailed {
        task: String,
        stderr: String,
    },
    Failed,
}

impl fmt::Display for TestCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTask { task, available } => write!(
                formatter,
                "unknown task `{}`; available tasks: {}",
                task,
                available.join(", ")
            ),
            Self::BuildSpawn { task, .. } => write!(formatter, "failed to build task `{task}`"),
            Self::BuildFailed { task, stderr } => {
                if stderr.trim().is_empty() {
                    write!(formatter, "failed to build task `{task}`")
                } else {
                    write!(
                        formatter,
                        "failed to build task `{task}`: {}",
                        stderr.trim()
                    )
                }
            }
            Self::Failed => formatter.write_str("one or more testcases failed"),
        }
    }
}

impl Error for TestCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BuildSpawn { source, .. } => Some(source),
            Self::UnknownTask { .. } | Self::BuildFailed { .. } | Self::Failed => None,
        }
    }
}

#[allow(dead_code)]
pub(crate) fn format_wrong_answer_diff(diff: &WrongAnswerDiff) -> String {
    let mut output = String::from("expected:\n");

    for line in diff.lines() {
        match line {
            WrongAnswerDiffLine::Different {
                line_number,
                expected,
                ..
            }
            | WrongAnswerDiffLine::MissingActual {
                line_number,
                expected,
            } => {
                output.push_str(&format!("{line_number:>4} | {expected}\n"));
            }
            WrongAnswerDiffLine::ExtraActual { line_number, .. } => {
                output.push_str(&format!("{line_number:>4} | <missing>\n"));
            }
        }
    }

    output.push_str("actual:\n");

    for line in diff.lines() {
        match line {
            WrongAnswerDiffLine::Different {
                line_number,
                actual,
                ..
            }
            | WrongAnswerDiffLine::ExtraActual {
                line_number,
                actual,
            } => {
                output.push_str(&format!("{line_number:>4} | {actual}\n"));
            }
            WrongAnswerDiffLine::MissingActual { line_number, .. } => {
                output.push_str(&format!("{line_number:>4} | <missing>\n"));
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use ac_core::output::compare_output;

    use super::{
        format_task_result, format_wrong_answer_diff, CaseDisplayResult, CaseOutcome,
        TaskDisplayResult,
    };

    #[test]
    fn formats_case_statuses_and_summary() {
        let result = TaskDisplayResult::new(
            "a",
            vec![
                CaseDisplayResult::new("sample1", CaseOutcome::Accepted),
                CaseDisplayResult::new("sample2", CaseOutcome::WrongAnswer { diff: None }),
                CaseDisplayResult::new(
                    "sample3",
                    CaseOutcome::RuntimeError {
                        exit_status: "exit code 101".to_owned(),
                        stderr: "panic at line 1\nbacktrace omitted".to_owned(),
                    },
                ),
                CaseDisplayResult::new("sample4", CaseOutcome::TimeLimitExceeded),
            ],
        );

        let formatted = format_task_result(&result).expect("formatting should succeed");

        assert_eq!(
            formatted,
            concat!(
                "task a\n",
                "[AC] sample1\n",
                "[WA] sample2\n",
                "[RE] sample3\n",
                "  exit status: exit code 101\n",
                "  stderr:\n",
                "    panic at line 1\n",
                "    backtrace omitted\n",
                "[TLE] sample4 timed out\n",
                "summary: AC 1 WA 1 RE 1 TLE 1 total 4\n",
            )
        );
    }

    #[test]
    fn formats_zero_case_summary() {
        let result = TaskDisplayResult::new("a", Vec::new());

        let formatted = format_task_result(&result).expect("formatting should succeed");

        assert_eq!(formatted, "task a\nsummary: AC 0 WA 0 RE 0 TLE 0 total 0\n");
    }

    #[test]
    fn formats_runtime_error_without_stderr_body_when_empty() {
        let result = TaskDisplayResult::new(
            "a",
            vec![CaseDisplayResult::new(
                "sample1",
                CaseOutcome::RuntimeError {
                    exit_status: "signal 9".to_owned(),
                    stderr: String::new(),
                },
            )],
        );

        let formatted = format_task_result(&result).expect("formatting should succeed");

        assert_eq!(
            formatted,
            concat!(
                "task a\n",
                "[RE] sample1\n",
                "  exit status: signal 9\n",
                "summary: AC 0 WA 0 RE 1 TLE 0 total 1\n",
            )
        );
    }

    #[test]
    fn formats_wrong_answer_diff_with_expected_and_actual_labels() {
        let comparison =
            compare_output(b"one\ntwo\nthree\n", b"one\nTWO\n").expect("comparison should succeed");
        let diff = comparison.wrong_answer_diff().expect("WA should have diff");

        let formatted = format_wrong_answer_diff(&diff);

        assert_eq!(
            formatted,
            "expected:\n   2 | two\n   3 | three\nactual:\n   2 | TWO\n   3 | <missing>\n"
        );
    }

    #[test]
    fn does_not_format_diff_for_accepted_comparison() {
        let comparison = compare_output(b"one\n", b"one").expect("comparison should succeed");

        assert!(comparison.wrong_answer_diff().is_none());
    }
}

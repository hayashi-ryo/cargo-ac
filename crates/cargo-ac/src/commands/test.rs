use ac_core::output::{WrongAnswerDiff, WrongAnswerDiffLine};

use crate::error::CliResult;

use std::io::{self, Write};

pub(crate) fn run(_task: String) -> CliResult {
    println!("`cargo ac test` is not implemented yet.");
    Ok(())
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
            CaseOutcome::WrongAnswer => {
                writeln!(writer, "[WA] {}", case.case_name())?;
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
    WrongAnswer,
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
                CaseOutcome::WrongAnswer => summary.wrong_answer += 1,
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
                CaseDisplayResult::new("sample2", CaseOutcome::WrongAnswer),
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

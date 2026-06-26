use ac_core::output::{WrongAnswerDiff, WrongAnswerDiffLine};

use crate::error::CliResult;

pub(crate) fn run(_task: String) -> CliResult {
    println!("`cargo ac test` is not implemented yet.");
    Ok(())
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

    use super::format_wrong_answer_diff;

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

use std::{error::Error, fmt, str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputComparison {
    expected: NormalizedOutput,
    actual: NormalizedOutput,
    matches: bool,
}

impl OutputComparison {
    fn new(expected: NormalizedOutput, actual: NormalizedOutput) -> Self {
        let matches = expected == actual;
        Self {
            expected,
            actual,
            matches,
        }
    }

    pub fn expected(&self) -> &NormalizedOutput {
        &self.expected
    }

    pub fn actual(&self) -> &NormalizedOutput {
        &self.actual
    }

    pub fn matches(&self) -> bool {
        self.matches
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedOutput {
    text: String,
}

impl NormalizedOutput {
    fn new(text: String) -> Self {
        Self { text }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

pub fn compare_output(
    expected: &[u8],
    actual: &[u8],
) -> Result<OutputComparison, OutputComparisonError> {
    let expected = normalize_output(expected, OutputSide::Expected)?;
    let actual = normalize_output(actual, OutputSide::Actual)?;

    Ok(OutputComparison::new(expected, actual))
}

fn normalize_output(
    output: &[u8],
    side: OutputSide,
) -> Result<NormalizedOutput, OutputComparisonError> {
    let output = str::from_utf8(output)
        .map_err(|source| OutputComparisonError::InvalidUtf8 { side, source })?;
    let mut normalized = output.replace("\r\n", "\n");

    if normalized.ends_with('\n') {
        normalized.pop();
    }

    normalized = normalized
        .split('\n')
        .map(|line| line.trim_end_matches([' ', '\t']))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(NormalizedOutput::new(normalized))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputSide {
    Expected,
    Actual,
}

impl fmt::Display for OutputSide {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expected => formatter.write_str("expected"),
            Self::Actual => formatter.write_str("actual"),
        }
    }
}

#[derive(Debug)]
pub enum OutputComparisonError {
    InvalidUtf8 {
        side: OutputSide,
        source: str::Utf8Error,
    },
}

impl OutputComparisonError {
    pub fn side(&self) -> OutputSide {
        match self {
            Self::InvalidUtf8 { side, .. } => *side,
        }
    }
}

impl fmt::Display for OutputComparisonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUtf8 { side, .. } => {
                write!(formatter, "{side} output is not valid UTF-8")
            }
        }
    }
}

impl Error for OutputComparisonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidUtf8 { source, .. } => Some(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{compare_output, OutputComparisonError, OutputSide};

    #[test]
    fn treats_lf_and_crlf_line_endings_as_equal() {
        let comparison =
            compare_output(b"hello\r\nworld\r\n", b"hello\nworld\n").expect("valid UTF-8");

        assert!(comparison.matches());
        assert_eq!(comparison.expected().as_str(), "hello\nworld");
        assert_eq!(comparison.actual().as_str(), "hello\nworld");
    }

    #[test]
    fn ignores_single_file_trailing_newline() {
        let comparison = compare_output(b"answer\n", b"answer").expect("valid UTF-8");

        assert!(comparison.matches());
        assert_eq!(comparison.expected().as_str(), "answer");
        assert_eq!(comparison.actual().as_str(), "answer");
    }

    #[test]
    fn ignores_trailing_spaces_and_tabs_on_each_line() {
        let comparison = compare_output(b"a \t\nb\t \n", b"a\nb\n").expect("valid UTF-8");

        assert!(comparison.matches());
        assert_eq!(comparison.expected().as_str(), "a\nb");
    }

    #[test]
    fn preserves_internal_spaces_tabs_and_token_order() {
        for (expected, actual) in [
            (&b"a b\n"[..], &b"a  b\n"[..]),
            (&b"a\tb\n"[..], &b"a b\n"[..]),
            (&b"1 2 3\n"[..], &b"1 3 2\n"[..]),
        ] {
            let comparison = compare_output(expected, actual).expect("valid UTF-8");

            assert!(!comparison.matches());
        }
    }

    #[test]
    fn preserves_extra_blank_lines() {
        for (expected, actual) in [
            (&b"a\n\nb\n"[..], &b"a\nb\n"[..]),
            (&b"a\n\n"[..], &b"a\n"[..]),
        ] {
            let comparison = compare_output(expected, actual).expect("valid UTF-8");

            assert!(!comparison.matches());
        }
    }

    #[test]
    fn returns_error_for_invalid_utf8_expected_output() {
        let error =
            compare_output(&[0xff], b"ok\n").expect_err("invalid expected UTF-8 should fail");

        assert_eq!(error.side(), OutputSide::Expected);
        assert!(matches!(error, OutputComparisonError::InvalidUtf8 { .. }));
    }

    #[test]
    fn returns_error_for_invalid_utf8_actual_output() {
        let error = compare_output(b"ok\n", &[0xff]).expect_err("invalid actual UTF-8 should fail");

        assert_eq!(error.side(), OutputSide::Actual);
        assert!(matches!(error, OutputComparisonError::InvalidUtf8 { .. }));
    }
}

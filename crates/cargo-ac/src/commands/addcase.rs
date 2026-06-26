use std::{
    error::Error,
    fmt, io,
    io::{BufRead, Write},
    path::Path,
};

use ac_core::{
    config::{ProjectConfig, TaskConfig},
    testcase::{add_custom_testcase, AddedTestcase},
};

use crate::error::CliResult;

const CONFIG_FILE: &str = "ac.toml";
const END_MARKER: &str = "---";

pub(crate) fn run(task: String) -> CliResult {
    let current_directory = std::env::current_dir()?;
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();

    let added = run_in_workspace(&current_directory, &task, &mut stdin, &mut stdout)?;
    writeln!(
        stdout,
        "Added testcase `{}`: {} {}",
        added.logical_name(),
        added.input_path().display(),
        added.output_path().display()
    )?;

    Ok(())
}

fn run_in_workspace(
    workspace_root: &Path,
    task_name: &str,
    reader: &mut impl BufRead,
    writer: &mut impl Write,
) -> Result<AddedTestcase, Box<dyn Error>> {
    let config = ProjectConfig::read(workspace_root.join(CONFIG_FILE))?;
    let task = find_task(&config, task_name)?;
    let input = read_block(
        reader,
        writer,
        "Input",
        "Enter input. Finish with a line containing only `---`.",
    )?;
    let expected_output = read_block(
        reader,
        writer,
        "Expected output",
        "Enter expected output. Finish with a line containing only `---`.",
    )?;

    add_custom_testcase(
        workspace_root,
        &config,
        task,
        input.as_bytes(),
        expected_output.as_bytes(),
    )
    .map_err(|error| Box::new(error) as Box<dyn Error>)
}

fn find_task<'a>(
    config: &'a ProjectConfig,
    task_name: &str,
) -> Result<&'a TaskConfig, AddcaseCommandError> {
    config
        .tasks()
        .iter()
        .find(|task| task.bin_name() == task_name)
        .ok_or_else(|| AddcaseCommandError::UnknownTask {
            task: task_name.to_owned(),
            available: config
                .tasks()
                .iter()
                .map(|task| task.bin_name().to_owned())
                .collect(),
        })
}

fn read_block(
    reader: &mut impl BufRead,
    writer: &mut impl Write,
    label: &str,
    prompt: &str,
) -> Result<String, AddcaseCommandError> {
    writeln!(writer, "{prompt}").map_err(AddcaseCommandError::Prompt)?;
    write!(writer, "{label}> ").map_err(AddcaseCommandError::Prompt)?;
    writer.flush().map_err(AddcaseCommandError::Prompt)?;

    let mut block = String::new();
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader
            .read_line(&mut line)
            .map_err(AddcaseCommandError::ReadInput)?;

        if bytes == 0 {
            return Err(AddcaseCommandError::UnexpectedEof {
                label: label.to_owned(),
            });
        }

        if line.trim_end_matches(['\r', '\n']) == END_MARKER {
            break;
        }

        block.push_str(&line);
    }

    Ok(block)
}

#[derive(Debug)]
enum AddcaseCommandError {
    UnknownTask {
        task: String,
        available: Vec<String>,
    },
    Prompt(io::Error),
    ReadInput(io::Error),
    UnexpectedEof {
        label: String,
    },
}

impl fmt::Display for AddcaseCommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTask { task, available } => write!(
                formatter,
                "unknown task `{}`; available tasks: {}",
                task,
                available.join(", ")
            ),
            Self::Prompt { .. } => formatter.write_str("failed to write addcase prompt"),
            Self::ReadInput { .. } => formatter.write_str("failed to read addcase input"),
            Self::UnexpectedEof { label } => {
                write!(formatter, "unexpected EOF while reading {label}")
            }
        }
    }
}

impl Error for AddcaseCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Prompt(source) | Self::ReadInput(source) => Some(source),
            Self::UnknownTask { .. } | Self::UnexpectedEof { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Cursor};

    use tempfile::tempdir;

    use super::{read_block, run_in_workspace};
    use ac_core::config::{ProjectConfig, TaskConfig};

    fn write_workspace(root: &std::path::Path) {
        fs::create_dir_all(root.join("testcases/a")).expect("testcase directory should be created");
        let config = ProjectConfig::new(
            "abc400",
            "src/bin",
            "testcases",
            "rust",
            "2021",
            vec![TaskConfig::new("abc400_a", "a")],
        );
        config
            .write(root.join("ac.toml"))
            .expect("config should be written");
    }

    #[test]
    fn reads_block_until_end_marker() {
        let mut reader = Cursor::new("1 2\n3 4\n---\n");
        let mut writer = Vec::new();

        let block =
            read_block(&mut reader, &mut writer, "Input", "Prompt").expect("block should be read");

        assert_eq!(block, "1 2\n3 4\n");
        assert!(String::from_utf8(writer)
            .expect("prompt should be UTF-8")
            .contains("Input> "));
    }

    #[test]
    fn addcase_writes_custom_pair_from_interactive_input() {
        let directory = tempdir().expect("temporary directory should be created");
        write_workspace(directory.path());
        let mut reader = Cursor::new("1 2\n---\n3\n---\n");
        let mut writer = Vec::new();

        let added = run_in_workspace(directory.path(), "a", &mut reader, &mut writer)
            .expect("custom testcase should be added");

        assert_eq!(added.logical_name(), "custom-1");
        assert_eq!(
            fs::read_to_string(directory.path().join("testcases/a/custom-1.in"))
                .expect("input should be readable"),
            "1 2\n"
        );
        assert_eq!(
            fs::read_to_string(directory.path().join("testcases/a/custom-1.out"))
                .expect("output should be readable"),
            "3\n"
        );
    }

    #[test]
    fn rejects_unknown_task() {
        let directory = tempdir().expect("temporary directory should be created");
        write_workspace(directory.path());
        let mut reader = Cursor::new("1\n---\n1\n---\n");
        let mut writer = Vec::new();

        let error = run_in_workspace(directory.path(), "missing", &mut reader, &mut writer)
            .expect_err("unknown task should fail");

        assert!(error.to_string().contains("available tasks: a"));
    }
}

use std::{
    error::Error,
    fmt, io,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRunRequest {
    workspace_root: PathBuf,
    task_name: String,
    profile: RunnerProfile,
    timeout: Duration,
}

impl TaskRunRequest {
    pub fn new(
        workspace_root: impl Into<PathBuf>,
        task_name: impl Into<String>,
        profile: RunnerProfile,
        timeout: Duration,
    ) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            task_name: task_name.into(),
            profile,
            timeout,
        }
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn task_name(&self) -> &str {
        &self.task_name
    }

    pub fn profile(&self) -> RunnerProfile {
        self.profile
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    fn binary_path(&self) -> PathBuf {
        self.workspace_root
            .join("target")
            .join(self.profile.target_directory())
            .join(executable_name(&self.task_name))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerProfile {
    Debug,
    Release,
}

impl RunnerProfile {
    fn target_directory(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskExecution {
    Finished(TaskExecutionOutput),
    TimedOut(TaskTimeoutOutput),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskExecutionOutput {
    exit_status: ProcessExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl TaskExecutionOutput {
    fn new(exit_status: ProcessExitStatus, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self {
            exit_status,
            stdout,
            stderr,
        }
    }

    pub fn exit_status(&self) -> ProcessExitStatus {
        self.exit_status
    }

    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }

    pub fn stderr(&self) -> &[u8] {
        &self.stderr
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTimeoutOutput {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl TaskTimeoutOutput {
    fn new(stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self { stdout, stderr }
    }

    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }

    pub fn stderr(&self) -> &[u8] {
        &self.stderr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessExitStatus {
    code: Option<i32>,
    success: bool,
}

impl ProcessExitStatus {
    fn from_std(status: std::process::ExitStatus) -> Self {
        Self {
            code: status.code(),
            success: status.success(),
        }
    }

    pub fn code(&self) -> Option<i32> {
        self.code
    }

    pub fn success(&self) -> bool {
        self.success
    }
}

pub fn run_task_binary(
    request: &TaskRunRequest,
    input: &[u8],
) -> Result<TaskExecution, TaskRunnerError> {
    let binary_path = request.binary_path();
    let mut command = Command::new(&binary_path);
    command
        .current_dir(request.workspace_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|source| TaskRunnerError::Spawn {
        workspace_root: request.workspace_root().to_path_buf(),
        binary_path,
        task_name: request.task_name().to_owned(),
        source,
    })?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| TaskRunnerError::OpenStdin {
            task_name: request.task_name().to_owned(),
        })?;
    stdin
        .write_all(input)
        .map_err(|source| TaskRunnerError::WriteStdin {
            task_name: request.task_name().to_owned(),
            source,
        })?;
    drop(stdin);

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| TaskRunnerError::OpenStdout {
            task_name: request.task_name().to_owned(),
        })?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| TaskRunnerError::OpenStderr {
            task_name: request.task_name().to_owned(),
        })?;
    let stdout_reader = read_pipe(stdout);
    let stderr_reader = read_pipe(stderr);
    let deadline = Instant::now() + request.timeout();

    loop {
        if let Some(status) = child.try_wait().map_err(|source| TaskRunnerError::Wait {
            task_name: request.task_name().to_owned(),
            source,
        })? {
            let stdout = join_pipe_reader(stdout_reader, PipeName::Stdout)?;
            let stderr = join_pipe_reader(stderr_reader, PipeName::Stderr)?;

            return Ok(TaskExecution::Finished(TaskExecutionOutput::new(
                ProcessExitStatus::from_std(status),
                stdout,
                stderr,
            )));
        }

        if Instant::now() >= deadline {
            child
                .kill()
                .map_err(|source| TaskRunnerError::KillTimedOut {
                    task_name: request.task_name().to_owned(),
                    source,
                })?;
            child.wait().map_err(|source| TaskRunnerError::Wait {
                task_name: request.task_name().to_owned(),
                source,
            })?;
            let stdout = join_pipe_reader(stdout_reader, PipeName::Stdout)?;
            let stderr = join_pipe_reader(stderr_reader, PipeName::Stderr)?;

            return Ok(TaskExecution::TimedOut(TaskTimeoutOutput::new(
                stdout, stderr,
            )));
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn read_pipe<T>(mut pipe: T) -> thread::JoinHandle<io::Result<Vec<u8>>>
where
    T: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut output = Vec::new();
        pipe.read_to_end(&mut output)?;
        Ok(output)
    })
}

fn join_pipe_reader(
    reader: thread::JoinHandle<io::Result<Vec<u8>>>,
    pipe: PipeName,
) -> Result<Vec<u8>, TaskRunnerError> {
    match reader.join() {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(source)) => Err(TaskRunnerError::ReadPipe { pipe, source }),
        Err(_) => Err(TaskRunnerError::ReadPipePanic { pipe }),
    }
}

fn executable_name(task_name: &str) -> String {
    if cfg!(windows) {
        format!("{task_name}.exe")
    } else {
        task_name.to_owned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeName {
    Stdout,
    Stderr,
}

impl fmt::Display for PipeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stdout => formatter.write_str("stdout"),
            Self::Stderr => formatter.write_str("stderr"),
        }
    }
}

#[derive(Debug)]
pub enum TaskRunnerError {
    Spawn {
        workspace_root: PathBuf,
        binary_path: PathBuf,
        task_name: String,
        source: io::Error,
    },
    OpenStdin {
        task_name: String,
    },
    WriteStdin {
        task_name: String,
        source: io::Error,
    },
    OpenStdout {
        task_name: String,
    },
    OpenStderr {
        task_name: String,
    },
    Wait {
        task_name: String,
        source: io::Error,
    },
    KillTimedOut {
        task_name: String,
        source: io::Error,
    },
    ReadPipe {
        pipe: PipeName,
        source: io::Error,
    },
    ReadPipePanic {
        pipe: PipeName,
    },
}

impl fmt::Display for TaskRunnerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spawn {
                workspace_root,
                binary_path,
                task_name,
                ..
            } => write!(
                formatter,
                "failed to spawn task `{task_name}` at `{}` in `{}`",
                binary_path.display(),
                workspace_root.display()
            ),
            Self::OpenStdin { task_name } => {
                write!(formatter, "failed to open stdin for task `{task_name}`")
            }
            Self::WriteStdin { task_name, .. } => {
                write!(formatter, "failed to write stdin for task `{task_name}`")
            }
            Self::OpenStdout { task_name } => {
                write!(formatter, "failed to open stdout for task `{task_name}`")
            }
            Self::OpenStderr { task_name } => {
                write!(formatter, "failed to open stderr for task `{task_name}`")
            }
            Self::Wait { task_name, .. } => {
                write!(formatter, "failed to wait for task `{task_name}`")
            }
            Self::KillTimedOut { task_name, .. } => {
                write!(formatter, "failed to stop timed out task `{task_name}`")
            }
            Self::ReadPipe { pipe, .. } => {
                write!(formatter, "failed to read task {pipe}")
            }
            Self::ReadPipePanic { pipe } => {
                write!(formatter, "task {pipe} reader failed unexpectedly")
            }
        }
    }
}

impl Error for TaskRunnerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Spawn { source, .. }
            | Self::WriteStdin { source, .. }
            | Self::Wait { source, .. }
            | Self::KillTimedOut { source, .. }
            | Self::ReadPipe { source, .. } => Some(source),
            Self::OpenStdin { .. }
            | Self::OpenStdout { .. }
            | Self::OpenStderr { .. }
            | Self::ReadPipePanic { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, process::Command, time::Duration};

    use tempfile::tempdir;

    use super::{run_task_binary, ProcessExitStatus, RunnerProfile, TaskExecution, TaskRunRequest};

    fn write_manifest(root: &std::path::Path) {
        fs::write(
            root.join("Cargo.toml"),
            r#"[package]
name = "runner_fixture"
version = "0.1.0"
edition = "2021"
"#,
        )
        .expect("manifest should be written");
    }

    fn write_bin(root: &std::path::Path, name: &str, source: &str) {
        let bin_directory = root.join("src/bin");
        fs::create_dir_all(&bin_directory).expect("bin directory should be created");
        fs::write(bin_directory.join(format!("{name}.rs")), source)
            .expect("binary source should be written");
    }

    fn request(root: &std::path::Path, task_name: &str, profile: RunnerProfile) -> TaskRunRequest {
        TaskRunRequest::new(root, task_name, profile, Duration::from_secs(5))
    }

    fn build_bin(root: &std::path::Path, name: &str, profile: RunnerProfile) {
        let mut command = Command::new(env!("CARGO"));
        command
            .current_dir(root)
            .arg("build")
            .arg("--quiet")
            .arg("--bin")
            .arg(name);

        if profile == RunnerProfile::Release {
            command.arg("--release");
        }

        let output = command.output().expect("cargo build should run");
        assert!(
            output.status.success(),
            "cargo build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn runs_debug_binary_with_stdin_and_captures_stdout_and_stderr() {
        let directory = tempdir().expect("temporary directory should be created");
        write_manifest(directory.path());
        write_bin(
            directory.path(),
            "echo",
            r#"use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    eprint!("stderr:");
    print!("stdout:");
    std::io::Write::write_all(&mut std::io::stdout(), &input).unwrap();
}
"#,
        );
        build_bin(directory.path(), "echo", RunnerProfile::Debug);

        let execution = run_task_binary(
            &request(directory.path(), "echo", RunnerProfile::Debug),
            b"hello\n",
        )
        .expect("task should run");

        let TaskExecution::Finished(output) = execution else {
            panic!("task should finish before timeout");
        };
        assert_eq!(
            output.exit_status(),
            ProcessExitStatus {
                code: Some(0),
                success: true,
            }
        );
        assert_eq!(output.stdout(), b"stdout:hello\n");
        assert_eq!(output.stderr(), b"stderr:");
    }

    #[test]
    fn runs_release_binary_when_requested() {
        let directory = tempdir().expect("temporary directory should be created");
        write_manifest(directory.path());
        write_bin(
            directory.path(),
            "profile",
            r#"fn main() {
    if cfg!(debug_assertions) {
        print!("debug");
    } else {
        print!("release");
    }
}
"#,
        );
        build_bin(directory.path(), "profile", RunnerProfile::Release);

        let execution = run_task_binary(
            &request(directory.path(), "profile", RunnerProfile::Release),
            b"",
        )
        .expect("task should run");

        let TaskExecution::Finished(output) = execution else {
            panic!("task should finish before timeout");
        };
        assert!(output.exit_status().success());
        assert_eq!(output.stdout(), b"release");
    }

    #[test]
    fn returns_non_zero_exit_as_execution_result() {
        let directory = tempdir().expect("temporary directory should be created");
        write_manifest(directory.path());
        write_bin(
            directory.path(),
            "fail",
            r#"fn main() {
    eprintln!("boom");
    std::process::exit(7);
}
"#,
        );
        build_bin(directory.path(), "fail", RunnerProfile::Debug);

        let execution = run_task_binary(
            &request(directory.path(), "fail", RunnerProfile::Debug),
            b"",
        )
        .expect("non-zero exit should still be an execution result");

        let TaskExecution::Finished(output) = execution else {
            panic!("task should finish before timeout");
        };
        assert_eq!(
            output.exit_status(),
            ProcessExitStatus {
                code: Some(7),
                success: false,
            }
        );
        assert_eq!(output.stdout(), b"");
        assert!(String::from_utf8_lossy(output.stderr()).contains("boom"));
    }

    #[test]
    fn returns_timeout_result() {
        let directory = tempdir().expect("temporary directory should be created");
        write_manifest(directory.path());
        write_bin(
            directory.path(),
            "sleepy",
            r#"use std::io::Write;

fn main() {
    println!("started");
    eprintln!("waiting");
    std::io::stdout().flush().unwrap();
    std::io::stderr().flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(30));
}
"#,
        );
        build_bin(directory.path(), "sleepy", RunnerProfile::Debug);
        let request = TaskRunRequest::new(
            directory.path(),
            "sleepy",
            RunnerProfile::Debug,
            Duration::from_secs(1),
        );

        let execution = run_task_binary(&request, b"").expect("timeout should be a result");

        let TaskExecution::TimedOut(_output) = execution else {
            panic!("task should time out");
        };
    }
}

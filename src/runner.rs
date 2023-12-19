use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Command, ExitStatus, Stdio},
    sync::mpsc::channel,
    thread,
};

use error_stack::{Context, Report, Result, ResultExt};

use crate::runner::error::ThreadError;

use self::error::{CommandRunError, SaveLogError};

mod error;

pub(crate) fn run_command<F, E>(
    mut command: Command,
    save_stdout: &str,
    save_stderr: &str,
    append: bool,
    formatter: F,
) -> Result<ExitStatus, CommandRunError>
where
    F: Fn(&str) -> Result<String, E>,
    E: Context,
{
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .change_context(CommandRunError)
        .attach_printable_lazy(|| format!("Failed to run command: {:?}", command))?;

    let stdout = child.stdout.take().ok_or(Report::new(CommandRunError))?;
    let stderr = child.stderr.take().ok_or(Report::new(CommandRunError))?;

    let (out_tx, out_rx) = channel();
    let (err_tx, err_rx) = channel();

    let out_thread = thread::spawn(move || -> Result<(), ThreadError> {
        let stdout_lines = BufReader::new(stdout).lines();
        for line in stdout_lines {
            let line = line.change_context(ThreadError)?;
            println!("{}", line);
            out_tx.send(line).change_context(ThreadError)?;
        }
        Ok(())
    });

    let err_thread = thread::spawn(move || -> Result<(), ThreadError> {
        let stderr_lines = BufReader::new(stderr).lines();
        for line in stderr_lines {
            let line = line.change_context(ThreadError)?;
            eprintln!("{}", line);
            err_tx.send(line).change_context(ThreadError)?;
        }
        Ok(())
    });

    let status = child.wait().change_context(CommandRunError)?;

    out_thread
        .join()
        .unwrap_or(Err(Report::new(ThreadError)))
        .change_context(CommandRunError)?;
    err_thread
        .join()
        .unwrap_or(Err(Report::new(ThreadError)))
        .change_context(CommandRunError)?;

    let stdout = out_rx.into_iter().collect::<Vec<String>>().join("\n");
    let stderr = err_rx.into_iter().collect::<Vec<String>>().join("\n");

    let stdout = &formatter(&stdout).change_context(CommandRunError)?;
    let stderr = &formatter(&stderr).change_context(CommandRunError)?;

    save(stdout, Path::new(save_stdout), append).change_context(CommandRunError)?;
    save(stderr, Path::new(save_stderr), append).change_context(CommandRunError)?;

    Ok(status)
}

fn save(content: &str, path: &Path, append: bool) -> Result<(), SaveLogError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .open(path)
        .change_context(SaveLogError)
        .attach_printable_lazy(|| format!("Failed to open {:?}", path))?;

    writeln!(file, "{}", content)
        .change_context(SaveLogError)
        .attach_printable_lazy(|| format!("Failed to write to {:?}", file))
}

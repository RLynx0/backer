use std::{
    io::{BufRead, BufReader, Read},
    process::{exit, Command, Stdio},
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use clap::Parser;
use config::{Backup, Context};

use crate::{
    config::{generate_settings, load_config, OutLvl},
    opt::Opt,
};

mod config;
mod opt;

fn main() {
    let opt = Opt::parse();
    check_rsync_exists();

    match opt.command {
        opt::Command::Run => run(),
        opt::Command::Manual {} => todo!(),
    }
}

fn run() {
    load_config()
        .and_then(|config| generate_settings(config))
        .unwrap_or_else(|e| {
            eprintln!("{:?}", e);
            exit(1)
        })
        .into_iter()
        .for_each(|(c, b)| run_backup(&c, b))
}

fn run_backup(context: &Context, backup: Backup) {
    let arguments = compile_args(&backup, context);

    let mut command = if backup.method.sudo {
        let mut c = Command::new("sudo");
        c.arg("rsync");
        c
    } else {
        Command::new("rsync")
    };

    let (status, stdout, stderr) = run_command(&mut command, arguments);

    dbg!(status, stdout.len(), stderr.len());
}

fn run_command(
    command: &mut Command,
    arguments: Vec<String>,
) -> (std::process::ExitStatus, String, String) {
    let mut child = command
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdout = child.stdout.take().unwrap();
    let child_stderr = child.stderr.take().unwrap();

    let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
    let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

    let stdout_thread = stdio_thread(child_stdout, stdout_tx);
    let stderr_thread = stdio_thread(child_stderr, stderr_tx);

    let status = child
        .wait()
        .expect("Internal error, failed to wait on child");

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    let stdout = stdout_rx.into_iter().collect::<Vec<String>>().join("");
    let stderr = stderr_rx.into_iter().collect::<Vec<String>>().join("");
    (status, stdout, stderr)
}

fn stdio_thread<T>(readable: T, tx: Sender<String>) -> JoinHandle<()>
where
    T: Read + Send + 'static,
{
    thread::spawn(move || {
        let stdout_lines = BufReader::new(readable).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            println!("{}", line);
            tx.send(line).unwrap();
        }
    })
}

fn compile_args(backup: &Backup, context: &Context) -> Vec<String> {
    let mut arguments = vec![String::from("-aAX")];

    if backup.method.dry_run {
        arguments.push(String::from("--dry-run"))
    }

    if backup.method.delete {
        arguments.push(String::from("--delete"))
    }

    match backup.output {
        OutLvl::Quiet => arguments.push(String::from("--quiet")),
        OutLvl::Verbose => arguments.push(String::from("--verbose")),
        _ => (),
    }

    arguments.extend(
        backup
            .exclude
            .iter()
            .map(|e| format!("--exclude={}", e.to_string(context).unwrap())),
    );

    arguments.push(backup.source.to_string(context).unwrap());
    arguments.push(backup.target.to_string(context).unwrap());

    arguments
}

fn check_rsync_exists() {
    let output = Command::new("rsync").arg("--version").output();
    if let Err(e) = output {
        eprintln!("Not able to execute rsync!\n=> {}", e);
        exit(1);
    }
}

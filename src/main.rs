use std::{
    error::Error,
    fmt,
    process::{exit, Command},
    str::FromStr,
};

use clap::Parser;
use config::Config;
use error_stack::{Result, ResultExt};
use fs::read_config;
use opt::Opt;

mod config;
mod ctx_string;
mod fs;
mod opt;
mod runner;

#[derive(Debug)]
struct FatalError;
impl Error for FatalError {}
impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encountered fatal error")
    }
}

fn main() {
    let opt = Opt::parse();
    check_rsync_exists();

    if let Err(e) = match opt.command {
        opt::Command::Run => run(),
        opt::Command::Preview => preview(),
        opt::Command::Configure { editor } => todo!(),
    } {
        eprintln!("{e:?}");
        exit(1);
    }
}

fn run() -> Result<(), FatalError> {
    let config = read_config().change_context(FatalError)?;
    let (shared_context, runners) = Config::from_str(&config)
        .change_context(FatalError)?
        .build()
        .change_context(FatalError)?;

    let runners = Vec::from_iter(runners.iter().enumerate().map(|(i, runner)| {
        let num = format!("{}/{}", i + 1, runners.len());
        println!("\n->> Now running {}\n", num);
        (num, runner.run(&shared_context))
    }));

    println!("\n\n\n[SUMMARY]\n");
    for (num, result) in runners {
        match result {
            Ok(((status, out, err), log_result)) => {
                println!(
                    "Backup {} : OK\n\
                    * {}\n\
                    * {} lines on stdout\n\
                    * {} lines on stderr\n",
                    num,
                    status,
                    out.lines().count(),
                    err.lines().count()
                );

                match log_result {
                    Ok(_) => println!("Log {} : OK\n", num),
                    Err(e) => eprintln!("Log {} : FAIL\n{:?}\n", num, e),
                }
            }
            Err(e) => eprintln!("Backup {} : FAIL\n{:?}\n", num, e),
        }
    }

    Ok(())
}

fn preview() -> Result<(), FatalError> {
    let config = read_config().change_context(FatalError)?;
    let (shared_context, runners) = Config::from_str(&config)
        .change_context(FatalError)?
        .build()
        .change_context(FatalError)?;

    for runner in runners {
        runner.preview(&shared_context);
    }

    Ok(())
}

fn check_rsync_exists() {
    let output = Command::new("rsync").arg("--version").output();
    if let Err(e) = output {
        eprintln!("Not able to execute rsync!\n=> {}", e);
        exit(1);
    }
}

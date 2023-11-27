use std::process::{exit, Command};

use clap::Parser;
use config::{Backup, ConfigError};
use error_stack::Result;

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
    let run = match get_run_settings() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{:?}", e);
            exit(1);
        }
    };

    for backup in run {
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
        arguments.extend(backup.exclude.iter().map(|e| format!("--exclude={}", e)));
        arguments.push(backup.source);
        arguments.push(backup.target);

        let output = Command::new("rsync")
            .args(arguments)
            .output()
            .expect("failed to run rsync");

        println!(
            "{}",
            String::from_utf8(output.stdout).expect("AAAAA couldn't read string")
        );
        eprintln!(
            "{}",
            String::from_utf8(output.stderr).expect("AAAAA couldn't read string")
        );
    }
}

fn get_run_settings() -> Result<Vec<Backup>, ConfigError> {
    generate_settings(load_config()?)
}

fn check_rsync_exists() {
    let output = Command::new("rsync").arg("--version").output();
    if let Err(e) = output {
        eprintln!("Not able to execute rsync!\n=> {}", e);
        exit(1);
    }
}

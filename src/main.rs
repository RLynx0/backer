use std::{
    env, fs,
    path::PathBuf,
    process::{exit, Command},
    str::FromStr,
};

use clap::Parser;
use config::Config;
use opt::Opt;

mod config;
mod ctx_string;
mod opt;
mod runner;

const CONFIG_FILE_NAME: &str = "backer.toml";

fn main() {
    let opt = Opt::parse();
    check_rsync_exists();

    match opt.command {
        opt::Command::Run => run(),
        opt::Command::Manual {} => todo!(),
    }
}

fn run() {
    let mut confpath = PathBuf::from(env::var("HOME").unwrap());
    confpath.push(".config");
    confpath.push(CONFIG_FILE_NAME);
    let config = fs::read_to_string(&confpath).unwrap();
    let (shared_context, runners) = Config::from_str(&config).unwrap().build().unwrap();
    for runner in runners {
        if let Err(e) = runner.run(&shared_context) {
            eprintln!("{e:#?}");
        }
    }
}

fn check_rsync_exists() {
    let output = Command::new("rsync").arg("--version").output();
    if let Err(e) = output {
        eprintln!("Not able to execute rsync!\n=> {}", e);
        exit(1);
    }
}

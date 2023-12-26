use std::path::PathBuf;

use clap::{command, Parser};

/// A configurable rsync wrapper with inbuilt logging
#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Opt {
    /// Path to config file
    #[arg(short, long, default_value = "~/.config/backer.toml")]
    config: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Parser)]
pub enum Command {
    /// Run configured backups
    Run,

    /// Preview configured backups
    Preview,

    /// Open config file
    Configure {
        #[arg(short, long, default_value = "vi")]
        editor: String,
    },
}

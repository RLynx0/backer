use clap::{command, Parser};

/// A slightly more concise wrapper for rsync with inbuilt log support
#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Opt {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Parser)]
pub enum Command {
    Run,
    Manual {},
}

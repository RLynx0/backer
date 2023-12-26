use std::process::{Command, ExitStatus};

use error_stack::{Result, ResultExt};

use crate::{
    ctx_string::{Context, CtxString},
    fs::SaveLogError,
    runner::run_command,
};

use super::{
    error::{BackupCompileError, BackupRunError},
    Backup, OutLvl, LOG_BINDING, SOURCE_BINDING, TARGET_BINDING,
};

const COMMAND_SUDO: &str = "sudo";
const COMMAND_RSYNC: &str = "rsync";
const BASE_ARGS: &str = "-aAx";
const ARG_DELETE: &str = "--delete";
const ARG_DRY_RUN: &str = "--dry-run";
const ARG_QUIET: &str = "--quiet";
const ARG_VERBOSE: &str = "--verbose";
const ARG_EXCLUDE: &str = "--exclude";

impl Backup {
    pub(crate) fn run(
        &self,
        variables: &Context,
    ) -> Result<((ExitStatus, String, String), Result<(), SaveLogError>), BackupRunError> {
        let (context, command, stdout, stderr) =
            self.compile(variables).change_context(BackupRunError)?;

        run_command(command, &stdout, &stderr, self.log.append, |log| {
            let mut context = context.clone();
            context.insert(LOG_BINDING.to_owned(), CtxString::literal(log));
            self.log.format.evaluate(&context)
        })
        .change_context(BackupRunError)
    }

    fn compile(
        &self,
        variables: &Context,
    ) -> Result<(Context, Command, String, String), BackupCompileError> {
        let mut context = variables.clone();
        context
            .entry(SOURCE_BINDING.to_owned())
            .or_insert(self.source.clone());
        context
            .entry(TARGET_BINDING.to_owned())
            .or_insert(self.target.clone());

        let mut args = self.as_args(&context)?.into_iter();
        let mut command = Command::new(args.next().unwrap());
        command.args(args);

        let stdout = self
            .log
            .stdout
            .evaluate(&context)
            .change_context(BackupCompileError)?;

        let stderr = self
            .log
            .stderr
            .evaluate(&context)
            .change_context(BackupCompileError)?;

        Ok((context, command, stdout, stderr))
    }

    fn as_args(&self, context: &Context) -> Result<Vec<String>, BackupCompileError> {
        let mut args = Vec::new();

        if self.method.sudo {
            args.push(COMMAND_SUDO.to_owned())
        }
        args.extend([COMMAND_RSYNC.to_owned(), BASE_ARGS.to_owned()]);
        if self.method.delete {
            args.push(ARG_DELETE.to_owned());
        }
        if self.method.dry_run {
            args.push(ARG_DRY_RUN.to_owned());
        }
        match self.output {
            OutLvl::Quiet => args.push(ARG_QUIET.to_owned()),
            OutLvl::Verbose => args.push(ARG_VERBOSE.to_owned()),
            _ => (),
        }
        for exclude in &self.exclude {
            args.push(format!(
                "{}={}",
                ARG_EXCLUDE,
                exclude
                    .evaluate(context)
                    .change_context(BackupCompileError)?
            ))
        }
        args.extend([
            self.source
                .evaluate(context)
                .change_context(BackupCompileError)?,
            self.target
                .evaluate(context)
                .change_context(BackupCompileError)?,
        ]);

        Ok(args)
    }
}

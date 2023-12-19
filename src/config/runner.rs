use std::process::{Command, ExitStatus};

use error_stack::{Result, ResultExt};

use crate::{
    ctx_string::{Context, CtxString},
    runner,
};

use super::{
    error::{BackupCompileError, BackupRunError},
    BackupRunner, OutLvl,
};

impl BackupRunner {
    pub(crate) fn run(&self, variables: &Context) -> Result<ExitStatus, BackupRunError> {
        let (context, command, stdout, stderr) =
            self.compile(variables).change_context(BackupRunError)?;

        Ok(
            runner::run_command(command, &stdout, &stderr, self.log.append, |log| {
                let mut context = context.clone();
                context.insert("log".to_string(), CtxString::literal(log));
                self.log.format.to_string(&context)
            })
            .change_context(BackupRunError)?,
        )
    }

    fn compile(
        &self,
        variables: &Context,
    ) -> Result<(Context, Command, String, String), BackupCompileError> {
        let mut context = variables.clone();
        for (key, val) in [
            ("source".to_string(), self.source.clone()),
            ("target".to_string(), self.target.clone()),
        ] {
            if !context.contains_key(&key) {
                context.insert(key, val);
            }
        }

        let mut args = self.as_args(&context)?.into_iter();
        let mut command = Command::new(args.next().unwrap());
        command.args(args);
        let stdout = self
            .log
            .stdout
            .to_string(&context)
            .change_context(BackupCompileError)?;
        let stderr = self
            .log
            .stderr
            .to_string(&context)
            .change_context(BackupCompileError)?;

        Ok((context, command, stdout, stderr))
    }

    fn as_args(&self, context: &Context) -> Result<Vec<String>, BackupCompileError> {
        let mut args = Vec::new();

        if self.method.sudo {
            args.push("sudo".to_string())
        }
        args.extend(["rsync".to_string(), "-aAx".to_string()]);
        if self.method.delete {
            args.push("--delete".to_string());
        }
        if self.method.dry_run {
            args.push("--dry-run".to_string());
        }
        match self.output {
            OutLvl::Quiet => args.push("--quiet".to_string()),
            OutLvl::Verbose => args.push("--verbose".to_string()),
            _ => (),
        }
        for exclude in &self.exclude {
            args.push(format!(
                "--exclude={}",
                exclude
                    .to_string(&context)
                    .change_context(BackupCompileError)?
            ))
        }
        args.extend([
            self.source
                .to_string(&context)
                .change_context(BackupCompileError)?,
            self.target
                .to_string(&context)
                .change_context(BackupCompileError)?,
        ]);

        Ok(args)
    }
}

use error_stack::{Result, ResultExt};

use super::{ctx_string::Context, error::BackupCompileError, BackupRunner, OutLvl};

impl BackupRunner {
    pub(crate) fn as_args(&self, variables: &Context) -> Result<Vec<String>, BackupCompileError> {
        let mut context = variables.clone();
        for (key, val) in [
            ("source".to_string(), self.source.clone()),
            ("target".to_string(), self.target.clone()),
        ] {
            if !context.contains_key(&key) {
                context.insert(key, val);
            }
        }

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

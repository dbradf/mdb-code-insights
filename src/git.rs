use std::{
    io::BufReader,
    path::{Path, PathBuf},
    process::{ChildStdout, Command, Stdio},
};

use anyhow::Result;

pub struct GitProxy {
    working_dir: PathBuf,
}

impl GitProxy {
    pub fn new(working_dir: &Path) -> Self {
        Self {
            working_dir: working_dir.to_path_buf(),
        }
    }

    pub fn log(&self, after_date: &str) -> Result<BufReader<ChildStdout>> {
        let dir = &self.working_dir;
        let mut command = Command::new("git")
            .args([
                "log",
                "--numstat",
                "--date=iso-strict",
                "--pretty=format:--%h--%cd--%aN--%s",
                "--no-renames",
                "--after",
                after_date,
            ])
            .current_dir(dir)
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(BufReader::new(command.stdout.take().unwrap()))
    }
}

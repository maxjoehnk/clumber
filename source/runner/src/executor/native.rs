use std::path::{Path, PathBuf};
use crate::executor::{Executor, Os, SpawnResult};
use std::process;

pub struct NativeExecutor {
    workdir: PathBuf
}

impl NativeExecutor {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            workdir: std::env::current_dir()?
        })
    }
}

impl Executor for NativeExecutor {
    fn os(&self) -> Os {
        Os::Linux
    }

    fn spawn(&self, command: &str) -> anyhow::Result<SpawnResult> {
        let mut command = command.split(' ');
        let exec = command.next().unwrap();
        let args = command;
        let child = process::Command::new(exec)
            .current_dir(&self.workdir)
            .args(args)
            .spawn()?;

        let output = child.wait_with_output()?;

        Ok(SpawnResult {
            status: output.status.code().unwrap() as i64,
            stdout: String::new(),
            stderr: String::new(),
        })
    }

    fn current_dir(&self) -> &Path {
        self.workdir.as_path()
    }

    fn change_dir(&mut self, dir: PathBuf) {
        self.workdir = dir;
    }
}

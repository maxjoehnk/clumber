use std::path::{Path, PathBuf};

pub mod native;
pub mod module;
pub mod docker;

pub trait Executor {
    fn os(&self) -> Os;
    fn spawn(&self, command: &str) -> anyhow::Result<SpawnResult>;

    fn current_dir(&self) -> &Path;
    fn change_dir(&mut self, dir: PathBuf);
}

pub enum Os {
    Linux,
    MacOs,
    Windows
}

impl<T: Executor + 'static> From<T> for Box<dyn Executor> {
    fn from(executor: T) -> Self {
        Box::new(executor)
    }
}

pub struct SpawnResult {
    status: i64,
    stdout: String,
    stderr: String,
}

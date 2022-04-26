use std::path::{Path, PathBuf};
use async_compat::CompatExt;
use bollard::container::Config;
use bollard::Docker;
use crate::executor::{Executor, Os, SpawnResult};
use futures::stream::StreamExt;

pub struct DockerExecutor {
    docker: Docker,
    image: String,
    workdir: PathBuf,
}

impl DockerExecutor {
    pub fn new(image: String) -> anyhow::Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;

        Ok(Self {
            docker,
            image,
            workdir: Path::new("/work").to_path_buf(),
        })
    }

    async fn spawn(&self, command: &str) -> anyhow::Result<SpawnResult> {
        let container = self.docker.create_container::<&str, &str>(None, Config {
            image: Some(self.image.as_str()),
            cmd: Some(command.split(' ').collect()),
            working_dir: Some(&self.workdir.to_str().unwrap_or_default()),
            ..Default::default()
        }).await?;
        self.docker.start_container::<&str>(&container.id, None).await?;

        // let status = self.docker.inspect_container(&container.id, None).await?;
        //
        // let attachment = self.docker.attach_container(&container.id, None).await?;

        let result = self.docker.wait_container::<&str>(&container.id, None).next().await.unwrap()?;

        self.docker.remove_container(&container.id, None).await?;

        Ok(SpawnResult {
            status: result.status_code,
            stdout: String::new(),
            stderr: String::new(),
        })
    }
}

impl Executor for DockerExecutor {
    fn os(&self) -> Os {
        Os::Linux
    }

    fn spawn(&self, command: &str) -> anyhow::Result<SpawnResult> {
        smol::block_on(self.spawn(command).compat())
    }

    fn current_dir(&self) -> &Path {
        self.workdir.as_path()
    }

    fn change_dir(&mut self, dir: PathBuf) {
        self.workdir = dir;
    }
}

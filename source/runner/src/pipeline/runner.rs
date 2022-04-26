use crate::{BuildContext, NativeExecutor};
use super::Pipeline;

pub struct PipelineRunner;

impl PipelineRunner {
    pub fn run(&self, pipeline: &Pipeline) -> anyhow::Result<()> {
        for stage in &pipeline.stages {
            let mut context = BuildContext::new(NativeExecutor::new()?);

            for step in &stage.steps {
                context.run_step(step)?;
            }
        }

        Ok(())
    }
}

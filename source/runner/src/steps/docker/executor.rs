use indexmap::IndexMap;
use serde_json::Value;
use crate::BuildContext;
use crate::executor::docker::DockerExecutor;
use crate::steps::{BuildStep, BuildStepFactory, BuildStepInterpreter};
use serde::Deserialize;

#[derive(Debug)]
pub struct DockerExecutorStep {
    pub image: String,
    pub steps: Vec<Box<dyn BuildStep>>
}

impl BuildStep for DockerExecutorStep {
    fn label(&self) -> String {
        format!("Run in docker {}", &self.image)
    }

    fn run(&self, context: &mut BuildContext) -> anyhow::Result<()> {
        let docker_executor = DockerExecutor::new(self.image.clone())?;
        let previous_executor = context.swap_executor(docker_executor);
        for step in &self.steps {
            context.run_step(step)?;
        }
        context.swap_executor(previous_executor);
        Ok(())
    }
}

pub struct DockerExecutorStepFactory;

impl BuildStepFactory for DockerExecutorStepFactory {
    fn identifier(&self) -> &'static str {
        "docker"
    }

    fn try_parse(&self, build_step_interpreter: &dyn BuildStepInterpreter, value: Value) -> anyhow::Result<Vec<Box<dyn BuildStep>>> {
        let raw_step: RawDockerExecutorStep = serde_json::from_value(value)?;

        Ok(vec![Box::new(DockerExecutorStep {
            image: raw_step.image,
            steps: build_step_interpreter.interpret(raw_step.steps)?
        }) as Box<dyn BuildStep>])
    }
}

#[derive(Debug, Deserialize)]
struct RawDockerExecutorStep {
    pub image: String,
    #[serde(default)]
    pub steps: IndexMap<String, Value>,
}

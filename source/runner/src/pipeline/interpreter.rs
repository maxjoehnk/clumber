use std::collections::HashMap;
use indexmap::IndexMap;
use serde_json::Value;
use crate::{ModuleLoader, Pipeline};
use crate::pipeline::parser::RawPipeline;
use crate::pipeline::Stage;
use crate::steps::{BuildStep, BuildStepFactory, BuildStepInterpreter};
use crate::steps::dir::DirStepFactory;
use crate::steps::docker::executor::DockerExecutorStepFactory;
use crate::steps::shell::ShellStepFactory;

pub struct PipelineInterpreter {
    module_loader: ModuleLoader,
}

impl PipelineInterpreter {
    pub fn new() -> anyhow::Result<Self> {
        let module_loader = ModuleLoader::new()?;

        Ok(Self {
            module_loader
        })
    }

    pub fn interpret(&mut self, raw_pipeline: RawPipeline) -> anyhow::Result<Pipeline> {
        let mut modules = vec![];
        for module_path in raw_pipeline.modules {
            let mut module = self.module_loader.load(&module_path)?;

            module.describe()?;

            modules.push(module);
        }

        let steps: HashMap<&str, Box<dyn BuildStepFactory>> = vec![
            Box::new(ShellStepFactory) as Box<dyn BuildStepFactory>,
            Box::new(DirStepFactory),
            Box::new(DockerExecutorStepFactory),
        ]
            .into_iter()
            .map(|factory| (factory.identifier(), factory))
            .collect();

        let step_interpreter = StepInterpreter {
            steps
        };

        let pipeline = Pipeline {
            modules,
            stages: raw_pipeline
                .stages
                .into_iter()
                .map(|(label, stage)| {
                    Ok(Stage {
                        label,
                        steps: step_interpreter.interpret(stage.steps)?,
                        parallel: vec![],
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        };

        Ok(pipeline)
    }
}

struct StepInterpreter<'a> {
    steps: HashMap<&'a str, Box<dyn BuildStepFactory>>
}

impl BuildStepInterpreter for StepInterpreter<'_> {
    fn interpret(&self, steps: IndexMap<String, Value>) -> anyhow::Result<Vec<Box<dyn BuildStep>>> {
        let steps = steps
            .into_iter()
            .map(|(identifier, step)| {
                let error = anyhow::anyhow!("Unknown step {}: {:?}", identifier, &step);
                if let Some(factories) = self.steps.get(&identifier.as_str()).map(|factory| factory.try_parse(self, step)) {
                    Ok(factories?)
                } else {
                    Err(error)
                }
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(steps)
    }
}

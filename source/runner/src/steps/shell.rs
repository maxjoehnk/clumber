use std::collections::HashMap;
use serde_json::Value;
use crate::steps::{BuildContext, BuildStep, BuildStepFactory, BuildStepInterpreter};
use serde::Deserialize;

#[derive(Debug)]
pub struct ShellStep {
    pub script: String,
    pub label: Option<String>,
    pub environment: HashMap<String, String>,
}

impl BuildStep for ShellStep {
    fn label(&self) -> String {
        self.label.clone().unwrap_or_else(|| self.script.clone())
    }

    fn run(&self, context: &mut BuildContext) -> anyhow::Result<()> {
        context.executor.spawn(&self.script)?;

        Ok(())
    }
}

pub struct ShellStepFactory;

impl BuildStepFactory for ShellStepFactory {
    fn identifier(&self) -> &'static str {
        "shell"
    }

    fn try_parse(&self, _: &dyn BuildStepInterpreter, value: Value) -> anyhow::Result<Vec<Box<dyn BuildStep>>> {
        if let Some(value) = value.as_object().cloned() {
            value.into_iter()
                .map(|(script, value)| {
                    let step: RawShellStep = serde_json::from_value(value)?;

                    Ok(Box::new(ShellStep {
                        script,
                        label: step.label,
                        environment: step.environment,
                    }) as Box<dyn BuildStep>)
                })
                .collect()
        }else {
            unreachable!()
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawShellStep {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

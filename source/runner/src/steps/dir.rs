use indexmap::IndexMap;
use serde_json::Value;
use crate::steps::{BuildContext, BuildStep, BuildStepFactory, BuildStepInterpreter};

#[derive(Debug)]
pub struct DirStep {
    pub dir: String,
    pub steps: Vec<Box<dyn BuildStep>>
}

impl BuildStep for DirStep {
    fn label(&self) -> String {
        format!("Change Directory {}", &self.dir)
    }

    fn run(&self, context: &mut BuildContext) -> anyhow::Result<()> {
        let old_dir = context.executor.current_dir().to_path_buf();
        context.executor.change_dir(old_dir.join(&self.dir));
        for step in &self.steps {
            context.run_step(step)?;
        }
        context.executor.change_dir(old_dir);
        Ok(())
    }
}

pub struct DirStepFactory;

impl BuildStepFactory for DirStepFactory {
    fn identifier(&self) -> &'static str {
        "dir"
    }

    fn try_parse(&self, build_step_interpreter: &dyn BuildStepInterpreter, value: Value) -> anyhow::Result<Vec<Box<dyn BuildStep>>> {
        if let Some(value) = value.as_object().cloned() {
            value.into_iter()
                .map(|(dir, value)| {
                    let steps: IndexMap<String, Value> = serde_json::from_value(value)?;

                    Ok(Box::new(DirStep {
                        dir,
                        steps: build_step_interpreter.interpret(steps)?
                    }) as Box<dyn BuildStep>)
                })
                .collect()
        }else {
            unreachable!()
        }
    }
}

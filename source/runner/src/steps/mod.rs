use std::fmt::Debug;
use indexmap::IndexMap;
use serde_json::Value;
use crate::executor::Executor;

pub mod dir;
pub mod docker;
pub mod shell;
pub mod module;

pub trait BuildStepFactory {
    fn identifier(&self) -> &'static str;

    fn try_parse(&self, build_step_interpreter: &dyn BuildStepInterpreter, value: Value) -> anyhow::Result<Vec<Box<dyn BuildStep>>>;
}

pub trait BuildStepInterpreter {
    fn interpret(&self, steps: IndexMap<String, Value>) -> anyhow::Result<Vec<Box<dyn BuildStep>>>;
}

pub trait BuildStep : Debug {
    fn label(&self) -> String;

    fn description(&self) -> String {
        Default::default()
    }

    fn run(&self, context: &mut BuildContext) -> anyhow::Result<()>;
}

pub struct BuildContext {
    pub executor: Box<dyn Executor>,
}

impl BuildContext {
    pub fn new(executor: impl Executor + 'static) -> Self {
        BuildContext {
            executor: Box::new(executor),
        }
    }

    pub fn swap_executor(&mut self, executor: impl Into<Box<dyn Executor>>) -> Box<dyn Executor> {
        let mut previous_executor = executor.into();
        std::mem::swap(&mut self.executor, &mut previous_executor);
        // TODO: set workdir of new executor?

        previous_executor
    }

    pub fn run_step(&mut self, step: impl AsRef<dyn BuildStep>) -> anyhow::Result<()> {
        let step = step.as_ref();
        println!("-- {}", step.label());
        step.run(self)?;

        Ok(())
    }
}

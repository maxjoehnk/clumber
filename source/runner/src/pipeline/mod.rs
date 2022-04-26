use crate::steps::BuildStep;
use crate::steps::module::ModuleRef;

pub mod parser;
pub mod runner;
pub mod interpreter;

#[derive(Debug)]
pub struct Pipeline {
    pub modules: Vec<ModuleRef>,
    pub stages: Vec<Stage>,
}

#[derive(Debug)]
pub struct Stage {
    pub label: String,
    pub steps: Vec<Box<dyn BuildStep>>,
    pub parallel: Vec<Stage>,
}

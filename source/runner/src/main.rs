use std::io::Read;
use crate::executor::native::NativeExecutor;
use crate::pipeline::parser::{PipelineParser, RawPipeline};
use crate::pipeline::Pipeline;
use crate::pipeline::interpreter::PipelineInterpreter;
use crate::pipeline::runner::PipelineRunner;
use crate::steps::BuildContext;
use crate::steps::module::ModuleLoader;
use clap::Parser;
use crate::flags::{Commands, Flags};

wit_bindgen_wasmtime::import!("./module.wit");

mod pipeline;
mod steps;
mod executor;
mod flags;

fn main() -> anyhow::Result<()> {
    let flags: Flags = Flags::parse();

    match flags.command {
        Commands::Run { pipeline } => run_pipeline(pipeline),
        Commands::Validate { pipeline } => validate_pipeline(pipeline),
        Commands::Agent => todo!()
    }
}

fn run_pipeline(file: String) -> anyhow::Result<()> {
    let pipeline = read_pipeline(&file)?;
    let mut interpreter = PipelineInterpreter::new()?;

    let pipeline = interpreter.interpret(pipeline)?;
    log::trace!("{:#?}", pipeline);

    PipelineRunner.run(&pipeline)?;

    Ok(())
}

fn validate_pipeline(file: String) -> anyhow::Result<()> {
    let pipeline = read_pipeline(&file)?;
    println!("{:#?}", pipeline);
    let mut interpreter = PipelineInterpreter::new()?;

    let pipeline = interpreter.interpret(pipeline)?;
    println!("{:#?}", pipeline);

    Ok(())
}

fn read_pipeline(file: &str) -> anyhow::Result<RawPipeline> {
    let mut file = std::fs::File::open(&file)?;
    let mut pipeline = String::new();
    file.read_to_string(&mut pipeline)?;

    let pipeline = PipelineParser.parse(&pipeline)?;

    Ok(pipeline)
}

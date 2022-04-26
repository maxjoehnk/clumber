use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Flags {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run {
        pipeline: String
    },
    Validate {
        pipeline: String
    },
    // TODO: implement agent mode
    Agent
}

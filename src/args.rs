use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[clap(name = "donitsi")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(name = "run")]
    Run(RunArgs),
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    pub path: String,
}
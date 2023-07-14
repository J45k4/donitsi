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
    #[clap(name = "ast")]
    Ast(AstArgs),
    #[clap(name = "donitsi")]
    Donitsi,
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    pub path: String,
}

#[derive(Debug, Parser)]
pub struct AstArgs {
    pub path: String
}
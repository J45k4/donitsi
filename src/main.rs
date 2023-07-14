use crate::args::Args;
use crate::args::Commands;
use clap::Parser;

mod parser;
mod parser_tests;
mod args;
mod commands;
mod window;
mod component;
mod vm;
mod types;
mod ui;
mod pretty;
mod donitsi;
mod components;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args: Args = Args::parse();

    match args.command {
        Commands::Run(run_args) => {
            commands::run(run_args);
        },
        Commands::Ast(ast_args) => {
            commands::ast(ast_args);
        },
        Commands::Donitsi => {
            commands::donitsi().await;
        }
    }
}

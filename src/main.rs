use crate::args::Args;
use crate::args::Commands;
use crate::parser::parse_code;
use clap::Parser;

mod parser;
mod parser_tests;
mod args;
mod commands;
mod window;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = Args::parse();

    match args.command {
        Commands::Run(run_args) => {
            commands::run(run_args);
        }
    }

    log::info!("Hello, world!");
}

/// Tokuin - Token usage and cost estimator for LLM prompts.
///
/// A fast CLI tool to estimate token usage and API costs for LLM prompts.
mod cli;
mod error;
mod models;
mod output;
mod parsers;
mod tokenizers;
mod utils;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

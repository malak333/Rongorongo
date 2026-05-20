pub mod audit;
pub mod cli;
pub mod model;
pub mod validate;
pub mod workflow;

use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    cli::Cli::parse().run()
}

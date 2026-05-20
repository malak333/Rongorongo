pub mod cli;
pub mod model;
pub mod validate;

use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    cli::Cli::parse().run()
}

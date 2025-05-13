use clap::Parser;
use cli::args::Args;

mod cli;
mod config;
mod error;
mod export;
mod prelude;

use crate::prelude::*;

fn main() -> Result<()> {
    let args = Args::parse();
    cli::run(args)
}

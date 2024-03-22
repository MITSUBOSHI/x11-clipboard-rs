mod cli;
mod error;
mod subcommand;

use crate::cli::{Cli, SubCommands};
use crate::error::Error;
use crate::subcommand::{copy, paste};
use clap::Parser;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Copy { data } => copy::run(data)?,
        SubCommands::Paste => paste::run()?,
    }

    Ok(())
}

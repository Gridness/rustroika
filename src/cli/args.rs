use std::fmt::Display;

use clap::{Parser, arg, command};
use clap_derive::Subcommand;
use evalexpr::eval_int;

use crate::prelude::*;

use crate::export::ExportFormat;

pub fn parse_expression(s: &str) -> Result<u32> {
    Ok(eval_int(s)
        .map_err(|_e| clap::Error::new(clap::error::ErrorKind::InvalidValue))
        .and_then(|i| {
            if i >= 0 {
                Ok(i as u32)
            } else {
                Err(clap::Error::new(clap::error::ErrorKind::InvalidValue))
            }
        })?)
}

#[derive(Parser)]
#[command(name = "rustroika", version, about, author, long_about = None, disable_help_flag = true)]
pub struct ConsoleCommand {
    #[command(subcommand)]
    pub command: Option<MainCommand>,

    #[arg(long, hide = true)]
    pub help: bool,
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Set configuration value
    Set {
        /// Key path in dot notation
        key: String,

        /// Value to set
        value: String,
    },

    /// Remove configuration value
    Remove {
        /// Key path in dot notation
        key: String,
    },

    /// Remove all configuration values
    Purge,
}

#[derive(Subcommand, Debug)]
pub enum MainCommand {
    /// Run the calculation
    Run {
        #[command(flatten)]
        params: RunParams,

        #[arg(long = "export", value_name = "FORMAT", value_enum)]
        export: Option<ExportFormat>,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Exit the console
    Exit,

    /// Clear the screen
    Clear,
}

impl Display for MainCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MainCommand::Run { .. } => "run",
            MainCommand::Config { .. } => "config",
            MainCommand::Exit => "exit",
            MainCommand::Clear => "clear",
        };
        write!(f, "{}", s)
    }
}

#[derive(Parser, Debug)]
pub struct RunParams {
    /// Number of trips per week (supports math expressions)
    #[arg(short = 't', long, value_parser=parse_expression)]
    pub trips_per_week: Option<u32>,

    /// Monthly pass cost (supports math expressions)
    #[arg(short = 'm', long, value_parser=parse_expression)]
    pub monthly_cost: Option<u32>,

    /// Ticket price (supports math expressions)
    #[arg(short = 'p', long, value_parser=parse_expression)]
    pub ticket_price: Option<u32>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<MainCommand>,
}

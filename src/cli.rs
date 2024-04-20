use crate::error::Error;
use clap::{Parser, Subcommand};
use std::io;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;

static STDIN_ALREADY_USED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Copy data from STDIN or command argument to X11 clipboard
    Copy {
        #[arg(default_value = "-")]
        data: Data,
    },
    /// Paste data from X11 clipboard to STDOUT
    Paste,
}

#[derive(Debug, Clone)]
pub enum Data {
    Stdin(String),
    Argument(String),
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Stdin(v) => write!(f, "{}", v),
            Data::Argument(v) => write!(f, "{}", v),
        }
    }
}

impl FromStr for Data {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => {
                if STDIN_ALREADY_USED.load(std::sync::atomic::Ordering::Acquire) {
                    return Err(Error::StdInAlreadyUsed);
                }
                STDIN_ALREADY_USED.store(true, std::sync::atomic::Ordering::SeqCst);
                let mut buffer = String::new();
                let stdin = io::stdin();

                for line in stdin.lines() {
                    match line {
                        Ok(line) => buffer.push_str(&format!("{}\n", line)),
                        Err(error) => return Err(Error::StdFailed(error)),
                    }
                }

                Ok(Self::Stdin(buffer.trim().to_string()))
            }
            arg => Ok(Self::Argument(arg.to_owned())),
        }
    }
}

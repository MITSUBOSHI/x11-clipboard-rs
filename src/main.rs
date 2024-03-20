use clap::{Parser, Subcommand};
use std::io::{self, BufWriter, Write};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;

static STDIN_ALREADY_USED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("failed to read stdin due to already used")]
    StdInAlreadyUsed,
    #[error("failed to copy data from clipboard: #{0}")]
    CopyFailed(String),
    #[error("failed to paste data to clipboard: #{0}")]
    PasteFailed(String),
    #[error(transparent)]
    StdFailed(#[from] std::io::Error),
    #[error("failed to open stdin")]
    StdinOpenFailed,
}

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Copy data from STDIN or command argument to X11 clipboard
    Copy {
        #[arg(default_value = "-")]
        data: Data,
    },
    /// Paste data from X11 clipboard to STDOUT
    Paste,
}

#[derive(Debug, Clone)]
enum Data {
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
                io::stdin()
                    .read_line(&mut buffer)
                    .map_err(Error::StdFailed)?;
                Ok(Self::Stdin(buffer.trim().to_string()))
            }
            arg => Ok(Self::Argument(arg.to_owned())),
        }
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Copy { data } => copy(data)?,
        SubCommands::Paste => paste()?,
    }

    Ok(())
}

fn copy(data: Data) -> Result<(), Error> {
    let data_string = data.to_string();
    let data = data_string.as_bytes();
    let mut child = Command::new("xsel")
        .args(&["--clipboard", "--input"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;

    let stdin = child.stdin.as_mut().ok_or(Error::StdinOpenFailed)?;
    stdin.write_all(data).map_err(Error::StdFailed)?;

    let status = child.wait();
    match status {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(Error::CopyFailed(format!("{}", status)))
            }
        }
        Err(e) => Err(Error::CopyFailed(format!("{}", e))),
    }
}

fn paste() -> Result<(), Error> {
    let output = Command::new("xsel")
        .args(&["--clipboard", "--output"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let out = io::stdout();
                let mut out = BufWriter::new(out.lock());
                write!(out, "{}", String::from_utf8(output.stdout).unwrap())?;
                Ok(())
            } else {
                Err(Error::PasteFailed(format!("{}", output.status)))
            }
        }
        Err(e) => Err(Error::PasteFailed(format!("{}", e))),
    }
}

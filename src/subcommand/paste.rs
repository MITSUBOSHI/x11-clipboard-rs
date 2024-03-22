use crate::error::Error;
use std::io::{self, BufWriter, Write};
use std::process::Command;

pub fn run() -> Result<(), Error> {
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

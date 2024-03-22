use std::io::Write;
use std::process::{Command, Stdio};

use crate::cli::Data;
use crate::error::Error;

pub fn run(data: Data) -> Result<(), Error> {
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

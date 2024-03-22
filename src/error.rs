#[derive(Debug, thiserror::Error)]
pub enum Error {
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

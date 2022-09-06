use thiserror::Error as ThisError;

pub type SlackResult<T> = Result<T, SlackError>;

/// Error enum.
#[derive(ThisError, Debug)]
pub enum SlackError {
    #[error("slack service error: {0}")]
    Slack(String),
    #[error("hex color parsing error: {0}")]
    HexColor(String),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Serialize(#[from] serde_json::error::Error),
    #[error(transparent)]
    FromHex(#[from] hex::FromHexError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    // #[error(transparent)]
    // Url(#[from] reqwest::UrlError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

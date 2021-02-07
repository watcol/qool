extern crate thiserror;

use thiserror::Error;

pub type QResult<T> = Result<T, QoolError>;

#[derive(Error, Debug)]
pub enum QoolError {
    #[error("File IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Log Initializing Error: {0}")]
    Log(#[from] log::SetLoggerError),
    #[error("Server Build Error: {0}")]
    Server(#[from] iron::error::HttpError),
    #[error("QR Code Writing Error: {0}")]
    QR(#[from] qr2term::QrError),
    #[error("Clipboard Reading Error: {0}")]
    Clipboard(#[from] Box<dyn std::error::Error>)
}

impl QoolError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::IO(e) => e.raw_os_error().unwrap_or(1),
            _ => 1
        }
    }
}

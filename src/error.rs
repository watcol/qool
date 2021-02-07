extern crate thiserror;

use thiserror::Error;

pub type QResult<T> = Result<T, QoolError>;

#[derive(Error, Debug)]
pub enum QoolError {
    #[error("File IO Error")]
    IO(#[from] std::io::Error),
    #[error("Log Initializing Error")]
    Log(#[from] log::SetLoggerError),
    #[error("Server Build Error")]
    Server(#[from] iron::error::HttpError),
    #[error("QR Code Writing Error")]
    QR(#[from] qr2term::QrError)
}

impl QoolError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::IO(e) => e.raw_os_error().unwrap_or(1),
            _ => 1
        }
    }
}

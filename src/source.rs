extern crate atty;

use std::fmt;

/// The Data Source
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// The text from the arguments.
    Text(String),
    /// The input file.
    File(String),
    /// Read from the standard input.
    Stdin,
    /// Redirected standard input.
    Redirected,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Source::Text(s) => write!(f, "{:?}", s),
            Source::File(s) => write!(f, "{}", s),
            Source::Stdin | Source::Redirected => write!(f, "<stdin>")
        }
    }
}

impl Source {
    /// Create `Source` with the arguments.
    pub fn new(text: Option<String>, file: Option<String>) -> Self {
        if let Some(t) = text {
            Self::Text(t)
        } else if let Some(f) = file {
            Self::File(f)
        } else if atty::is(atty::Stream::Stdin) {
            Self::Stdin
        } else {
            Self::Redirected
        }
    }

    /// Yield `Vec<u8>` from `Source`.
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Text(s) => Ok(text(s)),
            Self::File(s) => file(s),
            Self::Stdin => stdin(),
            Self::Redirected => redirected(),
        }.unwrap_or_else(|e| {
            log::error!("Failed to read: {}", e);
            std::process::exit(e.raw_os_error().unwrap_or(1));
        })
    }
}

fn text(s: String) -> Vec<u8> {
    s.into_bytes()
}

fn file(s: String) -> std::io::Result<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;

    let mut buf = Vec::new();
    File::open(s)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn stdin() -> std::io::Result<Vec<u8>> {
    use std::io::{self, Write};

    eprint!("Text to convert: ");
    io::stderr().flush()?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    Ok(s.trim().as_bytes().to_vec())
}

fn redirected() -> std::io::Result<Vec<u8>> {
    use std::io::{self, Read};

    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    Ok(buf)
}

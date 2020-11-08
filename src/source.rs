extern crate atty;
extern crate clipboard;
extern crate reqwest;

use std::fmt;

/// The Data Source
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// The text from the arguments.
    Text(String),
    /// The input file.
    File(String),
    /// Read from Clipboard.
    Clipboard,
    /// Link of the web pages.
    Link(String),
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
            Source::Clipboard => write!(f, "<clipboard>"),
            Source::Link(s) => write!(f, "{}", s),
            Source::Stdin | Source::Redirected => write!(f, "<stdin>"),
        }
    }
}

impl Source {
    /// Create `Source` with the arguments.
    pub fn new(text: Option<String>, file: Option<String>, clipboard: bool, link: Option<String>)
        -> Self {
        if let Some(t) = text {
            Self::Text(t)
        } else if let Some(f) = file {
            Self::File(f)
        } else if clipboard {
            Self::Clipboard
        } else if let Some(l) = link {
            Self::Link(l)
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
            Self::Clipboard => Ok(clipboard().unwrap_or_else(|e| {
                log::error!("Failed to read clipboard: {}", e);
                std::process::exit(1);
            })),
            Self::Link(s) => Ok(link(s).unwrap_or_else(|e| {
                log::error!("Failed to fetch link: {}", e);
                std::process::exit(1);
            })),
            Self::Stdin => stdin(),
            Self::Redirected => redirected(),
        }
        .unwrap_or_else(|e| {
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

fn clipboard() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use clipboard::{ClipboardContext, ClipboardProvider};
    Ok(ClipboardContext::new()?.get_contents()?.into_bytes())
}

fn link(s: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    tokio::runtime::Runtime::new()?.block_on(fetch(s))
}

async fn fetch(s: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::io::Write;

    let mut res = reqwest::get(&s).await?;
    let mut buf = Vec::new();

    while let Some(chunk) = res.chunk().await? {
        buf.write(&chunk)?;
    }

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

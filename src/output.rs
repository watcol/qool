use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Stdout,
    File(String),
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Target::Stdout => write!(f, "<stdout>"),
            Target::File(path) => write!(f, "{}", path),
        }
    }
}

impl From<Option<String>> for Target {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Target::File(s),
            None => Target::Stdout,
        }
    }
}

pub trait QoolWriter {
    fn qool_write(self, target: Target);
}

impl<T: Deref<Target = [u8]>> QoolWriter for T {
    fn qool_write(self, target: Target) {
        match target {
            Target::Stdout => stdout(&self),
            Target::File(path) => file(path, &self),
        }
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(e.raw_os_error().unwrap_or(1));
        });
    }
}

fn stdout(buf: &[u8]) -> io::Result<usize> {
    io::stdout().write(buf)
}

fn file(path: String, buf: &[u8]) -> io::Result<usize> {
    let mut file = File::create(path)?;
    file.write(buf)
}

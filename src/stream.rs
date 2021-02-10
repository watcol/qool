use crate::QResult;
use std::io::{Read, Stdin, copy};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub enum Stream {
    Buf(Vec<u8>),
    Stdin(Stdin),
    File(File),
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Buf(vec) => vec.as_slice().read(buf),
            Self::Stdin(stdin) => stdin.read(buf),
            Self::File(file) => file.read(buf),
        }
    }
}

impl Stream {
    pub fn buf<T: Into<Vec<u8>>>(buf: T) -> Self {
        Self::Buf(buf.into())
    }

    pub fn stdin() -> Self {
        Self::Stdin(std::io::stdin())
    }

    pub fn file<T: AsRef<Path>>(path: T) -> QResult<Self> {
        Ok(Self::File(File::open(path)?))
    }

    pub fn copy<T: AsRef<Path>>(&mut self, dst: T) -> QResult<()> {
        copy(self, &mut File::create(dst)?)?;
        Ok(())
    }
}

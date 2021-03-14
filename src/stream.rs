use crate::QResult;
use std::fs::File;
use std::io::{copy, Stdin, Write};
use std::path::Path;

#[derive(Debug)]
pub enum Stream {
    Buf(Vec<u8>),
    Stdin(Stdin),
    File(File),
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
        let mut dst = File::create(dst)?;
        match self {
            Self::Buf(vec) => dst.write(vec)? as u64,
            Self::Stdin(stdin) => copy(stdin, &mut dst)?,
            Self::File(file) => copy(file, &mut dst)?,
        };
        Ok(())
    }
}

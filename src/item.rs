extern crate clipboard;

use crate::{QResult, Stream};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Item {
    name: String,
    kind: ItemKind,
}

impl Item {
    pub fn stdin() -> Self {
        Self {
            name: String::from("stdin"),
            kind: ItemKind::Stdin,
        }
    }

    pub fn clipboard() -> Self {
        Self {
            name: String::from("clipboard"),
            kind: ItemKind::Clipboard,
        }
    }

    pub fn file<T: Into<PathBuf>>(path: T) -> QResult<Self> {
        let path = path.into();

        if path.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Can't upload a directory.",
            )
            .into());
        }

        let name = (&path).file_name().unwrap().to_string_lossy().into_owned();

        Ok(Self {
            name,
            kind: ItemKind::File(path),
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn copy<T: AsRef<Path>>(&self, dst: T) -> QResult<()> {
        self.kind.copy(dst)
    }
}

#[derive(Debug)]
enum ItemKind {
    Stdin,
    File(PathBuf),
    Clipboard,
}

impl ItemKind {
    pub fn copy<T: AsRef<Path>>(&self, dst: T) -> QResult<()> {
        let mut stream = match self {
            Self::Stdin => Stream::stdin(),
            Self::File(src) => Stream::file(src)?,
            Self::Clipboard => {
                let src = ClipboardContext::new()?.get_contents()?;
                Stream::buf(src.as_bytes())
            }
        };

        stream.copy(dst)?;
        Ok(())
    }
}

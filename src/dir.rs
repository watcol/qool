extern crate clipboard;
extern crate tempfile;

use crate::QResult;
use std::fs::{copy as fscopy, File};
use std::io::{copy as iocopy, stdin};
use std::path::{Path, PathBuf};
use clipboard::{ClipboardContext, ClipboardProvider};
use tempfile::TempDir;

#[derive(Debug)]
pub struct Directory {
    dir: TempDir,
    items: Vec<String>,
}

impl Directory {
    pub fn new() -> QResult<Self> {
        Ok(Self {
            dir: tempfile::tempdir()?,
            items: Vec::new(),
        })
    }

    pub fn add_stdin<T: Into<String>>(&mut self, name: T) -> QResult<&mut Self> {
        let path = self.add_name(name);
        iocopy(&mut stdin(), &mut File::create(path)?)?;
        Ok(self)
    }

    pub fn add_file<T: Into<String>>(&mut self, path: T) -> QResult<&mut Self> {
        let src = path.into();
        let name = match Path::new(&src).file_name() {
            Some(s) => s.to_string_lossy(),
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Can't upload a directory.",
            ).into()),
        };
        let dst = self.add_name(name);
        fscopy(src, dst)?;
        Ok(self)
    }

    pub fn add_clipboard<T: Into<String>>(&mut self, name: T) -> QResult<&mut Self> {
        let path = self.add_name(name);
        let buf = ClipboardContext::new()?.get_contents()?;
        let mut buf = buf.as_bytes();
        iocopy(&mut buf, &mut File::create(path)?)?;
        Ok(self)
    }

    pub fn path(&self) -> QResult<&Path> {
        self.add_buf("favicon.ico", include_bytes!("../assets/favicon.ico"))?
            .add_buf("logo.svg", include_str!("../assets/logo.svg"))?
            .add_buf("style.css", include_str!("../assets/style.css"))?
            .add_buf("index.html", self.build_index())?;

        Ok(self.dir.path())
    }

    fn add_name<T: Into<String>>(&mut self, name: T) -> PathBuf {
        let mut name = name.into();
        let presets: Vec<_> = ["favicon.ico", "logo.svg", "style.css", "index.html"]
            .iter()
            .map(|s| String::from(*s))
            .collect();

        while self.items.iter().chain(presets.iter()).any(|s| *s == name) {
            name.insert(0, '_');
        }

        let path = self.dir.path().join(&name);
        self.items.push(name.clone());
        path
    }

    fn build_index(&self) -> String {
        include_str!("../assets/index.html").replace(
            "{trs}",
            &self
                .items
                .iter()
                .map(|n| include_str!("../assets/_tr.html").replace("{name}", n))
                .collect::<Vec<_>>()
                .concat(),
        )
    }

    fn add_buf<T: Into<String>, U: AsRef<[u8]>>(&self, name: T, buf: U) -> QResult<&Self> {
        let name = name.into();
        let mut buf = buf.as_ref();
        let path = self.dir.path().join(&name);
        iocopy(&mut buf, &mut File::create(path)?)?;
        Ok(self)
    }
}

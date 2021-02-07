extern crate tempfile;

use crate::IORes;
use std::fs::File;
use std::io::{copy as iocopy, stdin};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[derive(Debug)]
pub struct Directory {
    dir: TempDir,
    items: Vec<String>,
}

impl Directory {
    pub fn new() -> IORes<Self> {
        Ok(Self {
            dir: tempfile::tempdir()?,
            items: Vec::new(),
        })
    }

    pub fn add_stdin<T: Into<String>>(&mut self, name: T) -> IORes<&mut Self> {
        let path = self.add_name(name);
        debug!("path: {:?}", path);
        iocopy(&mut stdin(), &mut File::create(path)?)?;
        Ok(self)
    }

    pub fn path(&self) -> IORes<&Path> {
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

    fn add_buf<T: Into<String>, U: AsRef<[u8]>>(&self, name: T, buf: U) -> IORes<&Self> {
        let name = name.into();
        let mut buf = buf.as_ref();
        let path = self.dir.path().join(&name);
        debug!("path: {:?}", path);
        iocopy(&mut buf, &mut File::create(path)?)?;
        Ok(self)
    }
}

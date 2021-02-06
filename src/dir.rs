extern crate tempfile;

use crate::IORes;
use std::fs::File;
use std::io::{copy as iocopy, stdin};
use std::path::PathBuf;

pub struct Directory {
    dir: PathBuf,
    items: Vec<String>,
}

impl Directory {
    pub fn new() -> IORes<Self> {
        let dir = tempfile::tempdir()?;

        Ok(Self {
            dir: PathBuf::from(dir.path()),
            items: Vec::new(),
        })
    }

    pub fn add_stdin<T: Into<String>>(mut self, name: T) -> IORes<Self> {
        let name = name.into();
        let path = (&self.dir).join(&name);
        iocopy(&mut stdin(), &mut File::create(path)?)?;
        self.items.push(name);
        Ok(self)
    }

    fn add_buf<T: Into<String>, U: AsRef<[u8]>>(self, name: T, buf: U) -> IORes<Self> {
        let name = name.into();
        let mut buf = buf.as_ref();
        let path = (&self.dir).join(&name);
        iocopy(&mut buf, &mut File::create(path)?)?;
        Ok(self)
    }

    pub fn path(self) -> IORes<PathBuf> {
        self.add_buf("favicon.ico", include_bytes!("../assets/favicon.ico"))?
            .add_buf("logo.svg", include_str!("../assets/logo.svg"))?
            .add_buf("style.css", include_str!("../assets/style.css"))?
            .add_buf(
                "index.html",
                include_str!("../assets/index.html").replace("{name}", &self.items[0]),
            )?;

        Ok(self.dir)
    }
}

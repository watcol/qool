use crate::{QResult, Item, Stream};
use std::path::PathBuf;

#[derive(Debug)]
pub struct DirBuilder {
    dir: PathBuf,
    items: Vec<String>,
}

impl DirBuilder {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self {
            dir: path.into(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) -> QResult<&mut Self> {
        let path = self.add_name(item.name());
        item.copy(path)?;
        Ok(self)
    }

    pub fn add_items<T: IntoIterator<Item=Item>>(&mut self, items: T) -> QResult<&mut Self> {
        for item in items {
            self.add_item(item)?;
        }

        Ok(self)
    }

    pub fn finalize(&self) -> QResult<()> {
        self.add_buf("favicon.ico", include_bytes!("../assets/favicon.ico"))?
            .add_buf("logo.svg", include_str!("../assets/logo.svg"))?
            .add_buf("style.css", include_str!("../assets/style.css"))?
            .add_buf("index.html", self.build_index())?;

        Ok(())
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

        let path = self.dir.join(&name);
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

    fn add_buf<T: AsRef<str>, U: AsRef<[u8]>>(&self, name: T, buf: U) -> QResult<&Self> {
        debug!("name: {}", name.as_ref());
        let path = self.dir.join(name.as_ref());
        Stream::buf(buf.as_ref()).copy(path)?;
        Ok(self)
    }
}

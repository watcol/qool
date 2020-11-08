use std::path::Path;
use std::fmt;
use std::io;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stream {
    File(String),
    Stdin
}

impl Default for Stream {
    fn default() -> Self {
        Self::Stdin
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::File(path) => write!(f, "{}", path),
            Self::Stdin => write!(f, "<stdin>"),
        }
    }
}

impl Stream {
    pub fn name(&self) -> String {
        match self {
            Self::File(path) => path.clone(),
            Self::Stdin => "file".to_string(),
        }
    }

    pub fn copy<T: AsRef<Path>>(&self, path2: T) -> io::Result<()> {
        match self {
            Self::File(path) => {fs::copy(path, path2)?;},
            Self::Stdin => {
                let mut file = fs::File::create(path2)?;
                io::copy(&mut io::stdin(), &mut file)?;
            }
        }

        Ok(())
    }
}

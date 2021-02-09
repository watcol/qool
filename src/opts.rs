extern crate atty;
extern crate fmtlog;
extern crate structopt;

use crate::QResult;
use crate::dir::Directory;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Opts {
    #[structopt(help = "Silence all log", long)]
    silent: bool,
    #[structopt(help = "Quiet log", short, long)]
    quiet: bool,
    #[structopt(help = "Verbose log", short, long)]
    verbose: bool,
    #[structopt(help = "Debug log", short, long)]
    debug: bool,
    #[structopt(help = "Change the log destination", short, long)]
    log: Option<std::path::PathBuf>,
    #[structopt(help = "Upload the contents in clipboard", short, long)]
    clipboard: bool,
    #[structopt(help = "A port to serve files", short, long, default_value = "3000")]
    port: u16,
    #[structopt(help = "The files to upload")]
    input: Vec<String>,
}

impl Opts {
    pub fn new() -> Self {
        Self::from_args()
    }

    pub fn silent(&self) -> bool {
        self.silent
    }

    pub fn quiet(&self) -> bool {
        self.silent
    }

    pub fn verbose(&self) -> bool {
        self.silent
    }

    pub fn debug(&self) -> bool {
        self.silent
    }

    pub fn log(&self) -> Option<std::path::PathBuf> {
        self.log.clone()
    }

    pub fn clipboard(&self) -> bool {
        self.clipboard
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn input(&self) -> Vec<String> {
        self.input.clone()
    }

    pub fn create_dir(&self) -> QResult<Directory> {
        let mut dir = Directory::new()?;

        // | len == 0 | clipboard | piped | read stdin |
        // |----------|-----------|-------|------------|
        // |        O |         O |     O |          O |
        // |        O |         O |     X |          X |
        // |        O |         X |     O |          O |
        // |        O |         X |     X |          O |
        // |        X |         O |     O |          O |
        // |        X |         O |     X |          X |
        // |        X |         X |     O |          O |
        // |        X |         X |     X |          X |
        if (self.input.len() == 0 && !self.clipboard) || atty::isnt(atty::Stream::Stdin) {
            dir.add_stdin("stdin")?;
        }

        if self.clipboard {
            dir.add_clipboard("clipboard")?;
        }

        self.input
            .iter()
            .fold(Ok(&mut dir), |dir, s| dir?.add_file(s))?;

        Ok(dir)
    }
}


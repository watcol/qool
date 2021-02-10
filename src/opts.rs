extern crate structopt;

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
}

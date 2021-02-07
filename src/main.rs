#[macro_use]
extern crate log;
extern crate atty;
extern crate fmtlog;
extern crate qr2term;
extern crate structopt;

mod dir;
mod error;
mod server;

use dir::Directory;
use error::QResult;
use server::Server;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
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
    fn init_log(&self) -> QResult<()> {
        use fmtlog::Config;

        let mut conf = Config::new()
            .level(self.log_level())
            .format(self.log_format());

        if let Some(ref path) = self.log {
            conf = conf.output(path);
        }

        fmtlog::new(conf).set()?;
        Ok(())
    }

    fn log_format(&self) -> &'static str {
        use fmtlog::formats::*;
        use fmtlog::LevelFilter::*;

        match self.log_level() {
            Off => "",
            Error | Warn => SIMPLE1,
            Info => DETAIL1,
            Debug | Trace => DEBUG1,
        }
    }

    fn log_level(&self) -> fmtlog::LevelFilter {
        use fmtlog::LevelFilter::*;

        if self.silent {
            Off
        } else if self.quiet {
            Error
        } else if !self.verbose && !self.debug {
            Warn
        } else if self.verbose && !self.debug {
            Info
        } else if !self.verbose && self.debug {
            Debug
        } else {
            Trace
        }
    }

    fn create_dir(&self) -> QResult<Directory> {
        let mut dir = Directory::new()?;

        if (self.input.len() == 0 && !self.clipboard) || atty::isnt(atty::Stream::Stdin) {
            dir.add_stdin("stdin")?;
        } else {
            self.input.iter().fold(Ok(&mut dir), |dir, s| dir?.add_file(s))?;
        }

        if self.clipboard {
            dir.add_clipboard("clipboard")?;
        }

        Ok(dir)
    }
}

fn print_url(url: String) -> QResult<()> {
    qr2term::print_qr(&url)?;
    println!("{}", url);
    Ok(())
}

fn inner_main() -> QResult<()> {
    let opts = Opts::from_args();
    opts.init_log()?;

    let dir = opts.create_dir()?;

    let server = Server::new(dir.path()?, opts.port)?;
    print_url(server.url())?;
    server.start()?;

    Ok(())
}

fn main() {
    inner_main().unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(e.exit_code());
    });
}

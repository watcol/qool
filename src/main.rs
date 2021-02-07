#[macro_use]
extern crate log;
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
    #[structopt(help = "A port to serve files", short, long, default_value = "3000")]
    port: u16,
}

fn init() -> QResult<Opts> {
    let opts = Opts::from_args();
    // fmtlog::new(fmtlog::Config::new().level(fmtlog::LevelFilter::Trace)).set()?;
    fmtlog::default().set()?;

    debug!("opts: {:?}", opts);
    Ok(opts)
}

fn print_url(url: String) -> QResult<()> {
    qr2term::print_qr(&url)?;
    println!("{}", url);
    Ok(())
}

fn inner_main() -> QResult<()> {
    let opts = init()?;

    let mut dir = Directory::new()?;
    let path = dir.add_stdin("stdin")?.path()?;

    let server = Server::new(path, opts.port)?;
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

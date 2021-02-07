#[macro_use]
extern crate log;
extern crate fmtlog;
extern crate qr2term;

mod dir;
mod error;
mod server;

use error::QResult;
use dir::Directory;
use server::Server;

fn init() -> QResult<()> {
    // fmtlog::new(fmtlog::Config::new().level(fmtlog::LevelFilter::Trace)).set()?;
    fmtlog::default().set()?;
    Ok(())
}

fn print_url(url: String) -> QResult<()> {
    qr2term::print_qr(&url)?;
    println!("{}", url);
    Ok(())
}

fn inner_main() -> QResult<()> {
    init()?;

    let mut dir = Directory::new()?;
    let path = dir.add_stdin("stdin")?.path()?;

    let server = Server::new(path)?;
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

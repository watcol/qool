#[macro_use]
extern crate log;
extern crate fmtlog;
extern crate qr2term;

mod dir;
mod server;

use std::io::Result as IORes;

use dir::Directory;
use server::Server;

fn init() {
    // fmtlog::new(fmtlog::Config::new().level(fmtlog::LevelFilter::Trace)).set().unwrap();
    fmtlog::default().set().unwrap();
}

fn print_url(url: String) {
    qr2term::print_qr(&url).unwrap_or_else(|e| {
        error!("Failed to print QR Code: {}", e);
        std::process::exit(1);
    });

    println!("{}", url);
}

fn inner_main() -> IORes<()> {
    init();

    let mut dir = Directory::new()?;
    let path = dir.add_stdin("stdin")?.path()?;

    let server = Server::new(path)?;
    print_url(server.url());
    server.start().unwrap_or_else(|e| {
        error!("Failed to build server: {}", e);
        std::process::exit(1);
    });

    Ok(())
}

fn main() {
    inner_main().unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(e.raw_os_error().unwrap_or(1));
    });
}

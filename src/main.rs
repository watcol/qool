#[macro_use]
extern crate log;
extern crate qr2term;

mod dir;
mod error;
mod opts;
mod server;

use error::QResult;
use opts::Opts;
use server::Server;

fn print_url(url: String) -> QResult<()> {
    qr2term::print_qr(&url)?;
    println!("{}", url);
    Ok(())
}

fn inner_main() -> QResult<()> {
    let opts = Opts::new();
    opts.init_log()?;

    let dir = opts.create_dir()?;
    let server = Server::new(dir.path()?, opts.port())?;
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

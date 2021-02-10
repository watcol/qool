#[macro_use]
extern crate log;
extern crate qr2term;

mod dir;
mod error;
mod opts;
mod log_builder;
mod server;

use dir::Directory;
use error::QResult;
use opts::Opts;
use log_builder::LogBuilder;
use server::Server;

fn print_url(url: String) -> QResult<()> {
    qr2term::print_qr(&url)?;
    println!("{}", url);
    Ok(())
}

fn inner_main() -> QResult<()> {
    let opts = Opts::new();

    LogBuilder::new()
        .silent(opts.silent())
        .quiet(opts.quiet())
        .verbose(opts.verbose())
        .debug(opts.debug())
        .log(opts.log())
        .init()?;

    let mut dir = Directory::new()?;
    let input = opts.input();
    let clipboard = opts.clipboard();

    if (input.len() == 0 && !clipboard) || atty::isnt(atty::Stream::Stdin) {
        dir.add_stdin("stdin")?;
    }

    if clipboard {
        dir.add_clipboard("clipboard")?;
    }

    input
        .iter()
        .fold(Ok(&mut dir), |dir, s| dir?.add_file(s))?;

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

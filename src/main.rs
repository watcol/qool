#[macro_use]
extern crate log;
extern crate qr2term;

mod dir;
mod error;
mod item;
mod opts;
mod log_builder;
mod server;
mod stream;

use dir::Directory;
use error::QResult;
use item::Item;
use opts::Opts;
use log_builder::LogBuilder;
use server::Server;
use stream::Stream;

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

    let mut items = Vec::new();
    let input = opts.input();
    let clipboard = opts.clipboard();

    if (input.len() == 0 && !clipboard) || atty::isnt(atty::Stream::Stdin) {
        items.push(Item::stdin());
    }

    if clipboard {
        items.push(Item::clipboard())
    }

    items.append(&mut input
        .iter()
        .map(|s| Item::file(s))
        .collect::<Result<Vec<_>, _>>()?);

    let mut dir = Directory::new()?;
    dir.add_items(items)?;

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

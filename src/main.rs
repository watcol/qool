#[macro_use]
extern crate log;
extern crate atty;
extern crate qr2term;

mod dir_builder;
mod error;
mod item;
mod log_builder;
mod opts;
mod server;
mod stream;

use dir_builder::DirBuilder;
use error::QResult;
use item::Item;
use log_builder::LogBuilder;
use opts::Opts;
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

    #[cfg(feature = "clipboard")]
    {
        let clipboard = opts.clipboard();

        if (input.is_empty() && !clipboard) || atty::isnt(atty::Stream::Stdin) {
            items.push(Item::stdin());
        }

        if clipboard {
            items.push(Item::clipboard());
        }
    }

    #[cfg(not(feature = "clipboard"))]
    {
        if input.is_empty() || atty::isnt(atty::Stream::Stdin) {
            items.push(Item::stdin())
        }
    }

    items.append(
        &mut input
            .iter()
            .map(Item::file)
            .collect::<Result<Vec<_>, _>>()?,
    );

    let server = Server::new(items, opts.port())?;
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

#[macro_use]
extern crate log;
extern crate tempfile;
extern crate iron;
extern crate staticfile;

mod file;
mod opts;

use tempfile::tempdir;
use iron::Iron;
use staticfile::Static;
use file::Stream;
use opts::init;

fn main() {
    inner_main().unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(e.raw_os_error().unwrap_or(1));
    })
}

fn inner_main() -> std::io::Result<()> {
    let opts = init();
    let stream = Stream::from(opts.input);

    let dir = tempdir()?;

    stream.copy(dir.path())?;

    Iron::new(Static::new(dir.path())).http("localhost:3000").unwrap_or_else(|e| {
        error!("Failed to build server: {}", e);
        std::process::exit(1);
    });

    Ok(())
}

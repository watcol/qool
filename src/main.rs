#[macro_use]
extern crate log;

mod file;
mod opts;

use file::Stream;
use opts::init;

fn main() {
    let opts = init();
    let stream = Stream::from(opts.input);

    stream.copy(".").unwrap_or_else(|e| {
        error!("Failed to copy file: {}", e);
        std::process::exit(e.raw_os_error().unwrap_or(1));
    });
}

#[macro_use]
extern crate log;
extern crate fmtlog;

use std::io::{stdin, Read, Result as IORes};

fn main() -> IORes<()> {
    //fmtlog::default()
    fmtlog::new(fmtlog::Config::new().level(log::LevelFilter::Trace))  // DEBUG
        .set()
        .unwrap();

    // Read from stdin.
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;

    // DEBUG
    debug!("buffer:\n{}", buf);

    Ok(())
}

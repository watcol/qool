#[macro_use]
extern crate log;
extern crate fmtlog;
extern crate tempfile;

use std::io::{stdin, Read, Write, Result as IORes};
use std::fs::File;

fn main() -> IORes<()> {
    //fmtlog::default()
    fmtlog::new(fmtlog::Config::new().level(log::LevelFilter::Trace))  // DEBUG
        .set()
        .unwrap();

    // Read from stdin.
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    debug!("buffer: {}", String::from_utf8_lossy(&buf));

    // Write to temporary file.
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("stdin");
    let mut file = File::create(&path)?;
    debug!("tempfile: {:?}", path.to_str());

    file.write_all(&buf)?;

    Ok(())
}

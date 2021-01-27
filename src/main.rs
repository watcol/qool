#[macro_use]
extern crate log;
extern crate fmtlog;
extern crate tempfile;
extern crate iron;
extern crate staticfile;
extern crate local_ipaddress;

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

    let ip = local_ipaddress::get().unwrap_or_else(|| {
        error!("Failed to get the ip address.");
        std::process::exit(1);
    });
    let port = 3000;

    let url = format!("http://{}:{}/{}", ip, port, "stdin");
    println!("{}", url);

    iron::Iron::new(staticfile::Static::new(dir.path()))
        .http((ip, port))
        .unwrap_or_else(|e| {
            error!("Failed to build server: {}", e);
            std::process::exit(1);
        });

    Ok(())
}

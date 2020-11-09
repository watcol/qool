#[macro_use]
extern crate log;
extern crate tempfile;
extern crate iron;
extern crate staticfile;
extern crate local_ipaddress;

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

    let ip = local_ipaddress::get().unwrap_or_else(|| {
        error!("Failed to get the ip address.");
        std::process::exit(1);
    });

    println!("http://{}:{}/{}", ip, opts.port, stream.name());

    Iron::new(Static::new(dir.path())).http((ip, opts.port)).unwrap_or_else(|e| {
        error!("Failed to build server: {}", e);
        std::process::exit(1);
    });

    Ok(())
}

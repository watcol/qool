#[macro_use]
extern crate log;
extern crate iron;
extern crate local_ipaddress;
extern crate qr2term;
extern crate staticfile;
extern crate tempfile;

mod file;
mod opts;

use file::Stream;
use iron::Iron;
use opts::init;
use staticfile::Static;
use tempfile::tempdir;

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

    qr2term::print_qr(format!("http://{}:{}/{}", ip, opts.port, stream.name())).unwrap_or_else(
        |e| {
            error!("Failed to print QR code: {}", e);
            std::process::exit(1);
        },
    );

    Iron::new(Static::new(dir.path()))
        .http((ip, opts.port))
        .unwrap_or_else(|e| {
            error!("Failed to build server: {}", e);
            std::process::exit(1);
        });

    Ok(())
}

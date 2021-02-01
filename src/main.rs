#[macro_use]
extern crate log;
extern crate fmtlog;
extern crate tempfile;
extern crate iron;
extern crate staticfile;
extern crate local_ipaddress;

use std::io::{stdin, Read, Write, Result as IORes};
use std::fs::File;

fn init() {
    //fmtlog::new(fmtlog::Config::new().level(log::LevelFilter::Trace))  // Debug
    fmtlog::default()
        .set()
        .unwrap();
}

fn read_buf() -> IORes<Vec<u8>> {
    // Read from stdin.
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    debug!("buffer: {}", String::from_utf8_lossy(&buf));
    Ok(buf)
}

fn add_file<T: AsRef<[u8]>>(dir: &std::path::Path, name: &str, buf: T) -> IORes<()> {
    let path = dir.join(name);
    let mut file = File::create(&path)?;
    file.write_all(buf.as_ref())?;
    Ok(())
}

fn create_dir<'a>() -> IORes<tempfile::TempDir> {
    let dir = tempfile::tempdir()?;
    let path = dir.path();
    let name = "stdin";
    add_file(path, name, read_buf()?)?;
    add_file(path, "favicon.ico", include_bytes!("../assets/favicon.ico"))?;
    add_file(path, "logo.svg", include_str!("../assets/logo.svg"))?;
    add_file(path, "style.css", include_str!("../assets/style.css"))?;
    add_file(path, "index.html", include_str!("../assets/index.html").replace("{name}", name))?;

    debug!("tempdir: {:?}", path.to_str());

    Ok(dir)
}

fn get_ip() -> String {
    let ip = local_ipaddress::get().unwrap_or_else(|| {
        error!("Failed to get the ip address.");
        std::process::exit(1);
    });
    debug!("IP Addr: {}", ip);

    ip
}

fn get_port() -> u16 {
    let port = 3000;
    debug!("Port: {}", port);

    port
}

fn print_url(ip: String, port: u16) {
    let url = format!("http://{}:{}", ip.clone(), port);
    println!("{}", url);
}

fn build_server(ip: String, port: u16, dir: &std::path::Path) {
    iron::Iron::new(staticfile::Static::new(dir))
        .http((ip, port))
        .unwrap_or_else(|e| {
            error!("Failed to build server: {}", e);
            std::process::exit(1);
        });
}

fn inner_main() -> IORes<()> {
    init();

    let dir = create_dir()?;
    let ip = get_ip();
    let port = get_port();

    print_url(ip.clone(), port);
    build_server(ip, port, dir.path());

    Ok(())
}

fn main() {
    inner_main().unwrap_or_else(|e| {
        error!("{}", e);
        std::process::exit(e.raw_os_error().unwrap_or(1));
    });
}

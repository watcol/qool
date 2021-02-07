extern crate iron;
extern crate staticfile;

use crate::QResult;
use std::net::{SocketAddr, UdpSocket};
use std::path::PathBuf;

use iron::Iron;
use staticfile::Static;

fn local_addr(port: u16) -> QResult<std::net::SocketAddr> {
    let socket = UdpSocket::bind(("0.0.0.0", port))?;
    socket.connect("8.8.8.8:80")?;
    Ok(socket.local_addr()?)
}

pub struct Server {
    addr: SocketAddr,
    dir: PathBuf,
}

impl Server {
    pub fn new<T: Into<PathBuf>>(path: T, port: u16) -> QResult<Self> {
        Ok(Self {
            addr: local_addr(port)?,
            dir: path.into(),
        })
    }

    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub fn start(&self) -> QResult<()> {
        Iron::new(Static::new(&self.dir)).http(self.addr)?;
        Ok(())
    }
}

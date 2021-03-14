extern crate iron;
extern crate staticfile;
extern crate tempfile;

use crate::{DirBuilder, Item, QResult};
use std::net::{SocketAddr, UdpSocket};
use tempfile::{tempdir, TempDir};

use iron::Iron;
use staticfile::Static;

fn local_addr(port: u16) -> QResult<std::net::SocketAddr> {
    let socket = UdpSocket::bind(("0.0.0.0", port))?;
    socket.connect("8.8.8.8:80")?;
    Ok(socket.local_addr()?)
}

pub struct Server {
    addr: SocketAddr,
    dir: TempDir,
}

impl Server {
    pub fn new<T: IntoIterator<Item = Item>>(items: T, port: u16) -> QResult<Self> {
        let dir = tempdir()?;
        debug!("dir: {:?}", dir.path());
        DirBuilder::new(dir.path()).add_items(items)?.finalize()?;

        Ok(Self {
            addr: local_addr(port)?,
            dir,
        })
    }

    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub fn start(&self) -> QResult<()> {
        Iron::new(Static::new(self.dir.path())).http(self.addr)?;
        Ok(())
    }
}

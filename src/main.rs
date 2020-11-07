extern crate log;
extern crate qrcode;

mod format;
mod opts;

use crate::{
    format::{Format, QoolRender},
    opts::init,
};
use qrcode::QrCode;
use std::io::{stdout, Write};

fn main() {
    let opts = init();

    let code = QrCode::new(opts.text).unwrap();
    let s = code.qool_render(opts.format);
    stdout().write(&s).unwrap_or_else(|e| {
        log::error!("{}", e);
        std::process::exit(1);
    });
}

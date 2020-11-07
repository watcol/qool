extern crate log;
extern crate qrcode;

mod format;
mod opts;
mod output;

use crate::{
    format::{Format, QoolRender},
    opts::init,
    output::{QoolWriter, Target},
};
use qrcode::QrCode;

fn main() {
    let opts = init();

    QrCode::new(opts.text)
        .unwrap_or_else(|e| {
            log::error!("Failed to generate QR code: {}", e);
            std::process::exit(1);
        })
        .qool_render(opts.format)
        .qool_write(opts.target);
}

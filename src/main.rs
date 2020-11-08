extern crate log;
extern crate qrcode;
extern crate byte_string;

mod format;
mod opts;
mod output;
mod source;

use crate::{
    format::{Format, QoolRender},
    opts::init,
    output::{QoolWriter, Target},
    source::Source,
};
use qrcode::QrCode;
use byte_string::ByteStr;

fn main() {
    let opts = init();

    let source = opts.source.into_bytes();
    log::trace!("Source Buffer: {:?}", ByteStr::new(&source));

    QrCode::new(source)
        .unwrap_or_else(|e| {
            log::error!("Failed to generate QR code: {}", e);
            std::process::exit(1);
        })
        .qool_render(opts.format)
        .qool_write(opts.target);
}

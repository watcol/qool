extern crate log;
extern crate qrcode;

mod format;
mod opts;
mod output;

use crate::{
    format::{Format, QoolRender},
    opts::init,
    output::{Target, QoolWriter},
};
use qrcode::QrCode;

fn main() {
    let opts = init();

    let code = QrCode::new(opts.text).unwrap();
    code.qool_render(opts.format).qool_write(opts.target);
}

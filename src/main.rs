extern crate colored;
extern crate log;
extern crate qrcode;
extern crate selog;

mod format;

use crate::format::{Format, QoolRender};
use qrcode::QrCode;
use std::io::{stdout, Write};

selog::opts! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Opts {
        #[clap(long, short = 'F', about = "The output format", possible_values = Format::VARIANTS,
               default_value = "term")]
        format: Format,
        #[clap(short, long, about = "The string to convert to QR code.")]
        text: String
    }
}

/// Initialize the application.
fn init() -> Opts {
    let opts = Opts::parse();
    opts.init_log().expect("Failed to initialize logger.");

    log::debug!("Output string: {}", opts.text);
    log::debug!("Output format: {}", opts.format);
    log::debug!("Output target: {}", "<stdout>");
    log::debug!("");

    opts
}

fn main() {
    let opts = init();

    let code = QrCode::new(opts.text).unwrap();
    let s = code.qool_render(opts.format);
    stdout().write(&s).unwrap_or_else(|e| {
        log::error!("{}", e);
        std::process::exit(1);
    });
}

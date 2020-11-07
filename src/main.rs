extern crate colored;
extern crate log;
extern crate qrcode;
extern crate selog;

mod format;

use crate::format::{Format, Output};
use qrcode::QrCode;
use std::io::{stdout, Write};

selog::opts! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Opts {
        #[clap(long, short, about = "The output format", possible_values = Format::VARIANTS,
               default_value = "term")]
        format: Format,
        #[clap(about = "The string to convert to QR code.", default_value = "Hello, World!")]
        input: String
    }
}

/// Initialize the application.
fn init() -> Opts {
    let opts = Opts::parse();
    opts.init_log().expect("Failed to initialize logger.");

    log::debug!("Output string: {}", opts.input);
    log::debug!("Output format: {}", opts.format);
    log::debug!("Output target: {}", "<stdout>");
    log::debug!("");

    opts
}

fn main() {
    let opts = init();

    let code = QrCode::new(opts.input).unwrap();
    let s = code.output(opts.format);
    stdout().write(&s).unwrap_or_else(|e| {
        log::error!("{}", e);
        std::process::exit(1);
    });
}

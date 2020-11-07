extern crate colored;
extern crate qrcode;
#[macro_use]
extern crate clap;

mod format;

use crate::format::{Format, Output};
use qrcode::QrCode;
use std::io::{stdout, Write};
use clap::Clap;

#[derive(Clone, Debug, PartialEq, Eq, Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
struct Opts {
    #[clap(about = "The string to convert to QR code.", default_value = "Hello, World!")]
    input: String
}

/// Initialize the application.
fn init() -> Opts {
    let opts = Opts::parse();

    println!("Output string: {}", opts.input);
    println!("Output format: {}", Format::Term);
    println!("Output target: {}", "<stdout>");
    println!();

    opts
}

fn main() {
    let opts = init();

    let code = QrCode::new(opts.input).unwrap();
    let s = code.output(Format::Term);
    stdout().write(&s).expect("Failed to write.");
}

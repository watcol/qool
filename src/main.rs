extern crate colored;
extern crate qrcode;

mod format;

use crate::format::{Format, Output};
use qrcode::QrCode;
use std::io::{stdout, Write};

fn main() {
    println!("Hello, world!");
    let code = QrCode::new(b"Hello, world!").unwrap();
    let s = code.output(Format::Term);
    stdout().write(&s).expect("Failed to write.");
}

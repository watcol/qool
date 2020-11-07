extern crate colored;
extern crate qrcode;

mod format;

use crate::format::{Format, Output};
use qrcode::QrCode;
use std::env::args;
use std::io::{stdout, Write};

fn main() {
    let original = args().nth(1).unwrap_or("Hello, World!".to_string());
    println!("Original: {}", original);
    let code = QrCode::new(original).unwrap();
    let s = code.output(Format::Term);
    stdout().write(&s).expect("Failed to write.");
}

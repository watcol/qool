extern crate colored;
extern crate qrcode;

use colored::Colorize;
use qrcode::QrCode;

fn main() {
    println!("Hello, world!");
    let code = QrCode::new("Hello, world!").unwrap();
    let s = code
        .render()
        .dark_color("  ".on_black().to_string().as_ref())
        .light_color("  ".on_white().to_string().as_ref())
        .build();
    println!("{}", s);
}

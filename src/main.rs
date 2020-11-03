extern crate qrcode;

use qrcode::QrCode;

fn main() {
    println!("Hello, world!");
    let code = QrCode::new("Hello, world!").unwrap();
    let s = code.render()
        .dark_color("\x1b[40m  \x1b[m")
        .light_color("\x1b[47m  \x1b[m")
        .build();
    println!("{}", s);
}

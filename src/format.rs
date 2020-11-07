extern crate colored;

use qrcode::QrCode;

/// The output formats.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Format {
    /// Display to terminal.
    Term,
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "term" => Ok(Self::Term),
            e => Err(format!("invalid string: {}", e)),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Term => write!(f, "term"),
        }
    }
}

impl Format {
    pub const VARIANTS: &'static [&'static str] = &["term"];
}

pub trait QoolRender {
    fn qool_render(self, f: Format) -> Vec<u8>;
}

impl QoolRender for QrCode {
    fn qool_render(self, f: Format) -> Vec<u8> {
        match f {
            Format::Term => term(self).into_bytes(),
        }
    }
}

fn term(code: QrCode) -> String {
    use colored::Colorize;

    code.render()
        .dark_color("  ".on_black().to_string().as_ref())
        .light_color("  ".on_white().to_string().as_ref())
        .build()
}

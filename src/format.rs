extern crate colored;

use qrcode::QrCode;
use image::ImageOutputFormat;

/// The output formats.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Format {
    /// Display to terminal.
    Term,
    /// PNG image.
    Png,
    /// JPEG image.
    Jpeg,
    /// GIF image.
    Gif,
    /// BMP image.
    Bmp,
    /// SVG image.
    Svg,
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "term" => Ok(Self::Term),
            "png" => Ok(Self::Png),
            "jpeg" => Ok(Self::Jpeg),
            "gif" => Ok(Self::Gif),
            "bmp" => Ok(Self::Bmp),
            "svg" => Ok(Self::Svg),
            e => Err(format!("invalid string: {}", e)),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Term => write!(f, "term"),
            Self::Png => write!(f, "png"),
            Self::Jpeg => write!(f, "jpeg"),
            Self::Gif => write!(f, "gif"),
            Self::Bmp => write!(f, "bmp"),
            Self::Svg => write!(f, "svg"),
        }
    }
}

impl Format {
    /// List of formats.
    pub const VARIANTS: &'static [&'static str] = &["term", "png", "jpeg", "gif", "bmp", "svg"];
}

/// Implement rendering with `Format` to `QRCode`.
pub trait QoolRender {
    fn qool_render(self, f: Format) -> Vec<u8>;
}

impl QoolRender for QrCode {
    fn qool_render(self, f: Format) -> Vec<u8> {
        match f {
            Format::Term => term(self).into_bytes(),
            Format::Png => image(self, ImageOutputFormat::Png),
            Format::Jpeg => image(self, ImageOutputFormat::Jpeg(0)),
            Format::Gif => image(self, ImageOutputFormat::Gif),
            Format::Bmp => image(self, ImageOutputFormat::Bmp),
            Format::Svg => svg(self).into_bytes(),
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

fn image(code: QrCode, format: ImageOutputFormat) -> Vec<u8> {
    let mut bytes = Vec::new();
    let img = code.render::<image::Luma<u8>>().build();
    let img = image::DynamicImage::ImageLuma8(img);

    img.write_to(&mut bytes, format).unwrap_or_else(|e| {
        log::error!("Failed to create image: {}", e);
        std::process::exit(1);
    });

    bytes
}

fn svg(code: QrCode) -> String {
    code.render::<qrcode::render::svg::Color>().build()
}

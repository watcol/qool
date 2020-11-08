mod file;

use std::env::args; // Debug

use file::Stream;

fn main() {
    let stream = args().nth(1).map(|s| Stream::File(s)).unwrap_or_default();

    stream.copy("file").unwrap();
}

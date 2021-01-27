use std::io::{stdin, Read, Result as IORes};

fn main() -> IORes<()> {
    // Read from stdin.
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;

    // DEBUG
    print!("{}", buf);

    Ok(())
}

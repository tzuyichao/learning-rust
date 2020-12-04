use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let _f = File::open("bar.txt")?;
    Ok(())
}
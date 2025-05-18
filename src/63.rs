use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    println!("Read from input.txt: {:?}", buffer);
    
    Ok(())
}

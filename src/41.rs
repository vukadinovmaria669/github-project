use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("example.txt")?;
    let lines: Vec<&str> = io::BufReader::new(file).lines().collect();

    for line in lines.iter() {
        println!("{}", line);
    }

    Ok(())
}

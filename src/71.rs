use std::fs;

fn main() {
    // Read a file and output its content to stdout
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
    println!("{}", contents);
}

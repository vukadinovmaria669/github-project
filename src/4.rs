use std::fs;

fn main() {
    let path = "/path/to/file";
    if !fs::metadata(path).is_dir() {
        println!("The file is not a directory.");
    } else {
        println!("The file is a directory.");
    }
}

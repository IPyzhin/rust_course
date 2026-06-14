use std::fs;
use std::io;

fn read_file(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}

fn main() {
    match read_file("viope.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Failed to read file: {}", e),
    }
}
use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let mut file = match OpenOptions::new().append(true).create(true).open("output.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open or create file: {}", e);
            return;
        }
    };
    let content = "\nLearning Rust is fun, but hard!";

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Content appended to file."),
        Err(e) => eprintln!("Failed to append content to file: {}", e),
    }
}

use std::fs::File;
use std::io::{self, Write};

fn main() {
    let mut file = match File::create("words.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    };

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        println!("Enter a word (type 'end' to finish): ");

        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line");
        buffer = buffer.trim().to_string();

        if buffer == "end" {
            println!("Word entry completed.");
            break;
        }

        // Write the word to the file
        if let Err(e) = writeln!(file, "{}", buffer) {
            eprintln!("Failed to write to file: {}", e);
            return;
        }
    }
}

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <operation> <num1> <num2>", args[0]);
        process::exit(1);
    }

    let operation = &args[1];

    let num1: i32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Second argument is not a number");
            process::exit(1);
        }
    };

    let num2: i32 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Third argument is not a number");
            process::exit(1);
        }
    };

    match operation.as_str() {
        "add" => println!("Result: {}", num1 + num2),
        "sub" => println!("Result: {}", num1 - num2),
        _ => {
            eprintln!("Unsupported operation: {}", operation);
            process::exit(1);
        }
    }
}
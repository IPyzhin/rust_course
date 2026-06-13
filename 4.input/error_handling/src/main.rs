use std::io;

fn main() {
    //Teoretical
    println!("Please enter your name");
    let mut name = String::new();
    let result = io::stdin().read_line(&mut name);

    match result {
        Ok(_) => {
            println!("Hello, {}!", name.trim());
        }
        Err(error) => {
            eprintln!("Erro reading inpit: {}", error);
        }
    }

    //2.1 Print strings
    let mut s = String::new();
    let mut num = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut s).expect("Failed to read the string");
    println!("Enter a number");
    let result = io::stdin().read_line(&mut num);
    
    match result {
        Ok(_) => {
            let num = num.trim().parse().expect("Number shoudnt be string");
            for _ in 1..=num {
                print!("{}", s)
            }
        }
        Err(_) => {
            eprintln!("You didn't enter a valid number!")
        }
    }
    //2.2 Multiplication table
    let mut n = String::new();
    println!("Enter an integer: ");
    io::stdin().read_line(&mut n).expect("Failed to read input");

    let n: u32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid integer.");
            return;
        }
    };

    for i in 1..=10 {
        println!("{} x {} = {}", n, i, n*i);
    } 

}


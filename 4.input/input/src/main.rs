use std::io;

fn main() {
    println!("Please enter your number:");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error when reading");
    
    let num: u32 = num.trim().parse().expect("Please enter number");
    println!("The number entered was {num}");  
}

use rand::RngExt;
use chrono::Local;

fn main() {
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..999);
    println!("Random number: {}", n);
    
    let local_time = Local::now();
    let time = local_time.format("%Y-%m-%d %H:%M:%S");
    println!("{}", time);
}
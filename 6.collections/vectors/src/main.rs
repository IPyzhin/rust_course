use std::io::stdin;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("Please enter 5 integers:");
    for _ in 0..5 {
        let mut n = String::new();
        stdin().read_line(&mut n).expect("Failed to read input");
        let n: i32 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid integer.");
                return;
            }
        };
        v.push(n);
    }
    v.sort();
    println!("Sorted vector: {:?}", v);
    let vv = vec![1, 2, 3, 4, 5];
    println!("The first element is: {}", vv[0]);
    println!("The length of the vector is: {}", vv.len());
}

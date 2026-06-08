fn main() {
    // Calculate the sum of even numbers from 1 to 10
    let start = 1;
    let end = 10;
    let mut sum = 0;

    for i in start..=end {
        if i % 2 == 0 {
            sum += i;
        }
    }

    println!("Sum of the even numbers from {start} to {end}: {sum}");
    //sum calc
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("Sum: {sum}");
    //countdown
    let mut c = 10;
    while c >= 1 {
        println!("{c}");
        c -= 1;
    }
    println!("Countdown");
}
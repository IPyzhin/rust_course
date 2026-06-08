fn main() {
    //Even or Odd
    let n: i64 = 7;
    if n % 2 == 0 {
        println!("{n} is even");
    } else {
        println!("{n} is odd");
    }
    //Largest number
    let a = 10;
    let b = 15;
    let c = 4;
    if a >= b && a >= c {
        println!("The largest number is: {a}");
    }
    else if b >= a && b >= c {
        println!("The largest number is: {b}");
    }
    else {
        println!("The largest number is: {c}");
    }
    //Grades
    let score = 91;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("The grade classification is: {grade}");
}

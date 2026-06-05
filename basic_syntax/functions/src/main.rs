fn main() {
    // Define a function that takes a name as an argument and returns a greeting message
    let name = "Mary";
    println!("{}", greet(name));

    fn greet(name: &str) -> String {
        format!("Hello {name}!")
    }
    //factorial function
    let num = 10;
    println!("Factorial of {num} is: {}", factorial(num));
    
    fn factorial(n: i32) -> i32 {
        let mut sum = 1;
        for i in 1..=n {
            sum *= i 
        }
        sum
    }

    let x = 5;
    let x = x + 1;
    println!("{}", x);
}

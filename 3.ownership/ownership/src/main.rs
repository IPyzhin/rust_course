fn main() {
    let msg1 = String::from("Hello"); // msg1 becomes the owner of the string.
    let msg2 = msg1; // Ownership is moved from msg1 to msg2.

    println!("{}", msg1); // Cause an error, msg1 no longer owns the string.
    println!("{}", msg2); // This is correct, msg2 now owns the string.
    
    let greeting = String::from("Hello, Rust!");
    print_length(greeting);

    /* 
     This line will cause a compile-time error because 
     greetings's ownership was moved to function 
    */
    println!("The length of '{}' is {}.", greeting, greeting.len());
}

fn print_length(s: String) {
    println!("The length of '{}' is {}.", s, s.len());
}
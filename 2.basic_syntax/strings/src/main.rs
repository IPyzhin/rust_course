fn main() {
    let o_string = "I like Python Programming";
    let mut m_string = o_string.replace("Python", "Rust");
    println!("Original string: {}", o_string);
    println!("Modified string: {}", m_string);
    println!("Length of the original string: {}", o_string.len());
    println!("Length of the modified string: {}", m_string.len());
}

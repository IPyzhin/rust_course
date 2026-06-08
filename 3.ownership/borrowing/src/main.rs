fn main() {
    let mut x = 10;
    let y = &mut x;
    println!("{y}");
    *y = 7; // You can modify x through y
    println!("{x}");

    let mut msg1 = String::from("Hello ");
    let msg2 = &mut msg1;
    msg2.push_str("world");
    println!("{msg2}");
    
    //2.1 Print length
    let st = String::from("Hello World!");
    let l = print_length(&st);
    println!("The length of {} is {}", st, l);
    println!("Original string: {st}");

    fn print_length(s: &String) -> usize {
        s.len()
    }
    //2.2 Mutable reference

    let mut s = String::from("Hello ");
    append_world(&mut s);
    println!("{}", s);

    fn append_world(s: &mut String) {
        s.push_str("world");
    }

    let s = String::from("Hello World!");
    let t = s;
    println!("{}", s);
}
use std::fmt::Debug;

struct Rectangle<T, U> {
    width: T,
    height: U,
}

fn print_display<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn main() {
    print_display(5); // prints 5
    print_display("Hello"); // prints "Hello"
    let rect = Rectangle { width: 5, height: 10.5 };
    println!("width {}, height {}", rect.width, rect.height);
}

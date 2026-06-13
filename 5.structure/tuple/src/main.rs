fn main() {
    //Tuple operations
    let tp = (42, 3.14, "Hello");
    println!("Integer: {}", tp.0);
    println!("Float: {}", tp.1);
    println!("String: {}", tp.2);
    let (x, y, z) = tp;
    println!("Destructured Integer: {}", x);
    println!("Destructured Float: {}", y);
    println!("Destructured String: {}", z);
    //Tuples in functions
    let t = (4.5, 7.5);
    println!("Sum: {}", func(t.0, t.1).0);
    println!("Difference: {}", func(t.0, t.1).1);
    

    fn func(a: f32, b: f32) -> (f32, f32) {
        let sum = a + b;
        let dif = a - b;
        (sum, dif)
    }
}

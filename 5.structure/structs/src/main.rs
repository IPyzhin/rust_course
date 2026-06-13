struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: String, author: String, pages: u32) -> Book {
        Book { title, author, pages }
    }

    fn print_info(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Pages: {}", self.pages);
    }
}
    


fn main() {
    let mut book1 = Book::new(String::from("Rust Programming"), String::from("John Smith"), 432);
    book1.print_info();
    book1.pages = 500;
    println!("Updated Pages: {}", book1.pages);
}

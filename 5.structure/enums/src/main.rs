#[derive(Debug)]
enum Genre {
    Fiction,
    Science,
    History
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    genre: Genre,
}

impl Book {
    fn new(title: String, author: String, pages: u32, genre: Genre) -> Book {
        Book { title, author, pages, genre }
    }

    fn print_info(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("Pages: {}", self.pages);
        println!("Genre: {:?}", self.genre);
    }
}
    


fn main() {
    let mut book1 = Book::new(String::from("Rust Programming"), String::from("John Smith"), 432, Genre::Science);
    let mut book2 = Book::new(String::from("Rust Programming"), String::from("John Smith"), 432, Genre::Fiction);
    let mut book3 = Book::new(String::from("Rust Programming"), String::from("John Smith"), 432, Genre::History);
    book1.print_info();
    println!("{:?}", book1);
}

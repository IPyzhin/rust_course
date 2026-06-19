trait Summary {
    fn summarize(&self);
}

struct Person {
    name: String,
    age: u8,
}

impl Summary for Person {
    fn summarize(&self) {
        println!("Person: Name = {}, Age = {}", self.name, self.age);
    }
}

struct Company {
    name: String,
    industry: String,
}

impl Summary for Company {
    fn summarize(&self) {
        println!("Company: Name = {}, Industry = {}", self.name, self.industry);
    }
}

fn main() {
    let person = Person { name: "Mary".to_string(), age: 35 };
    let company = Company { name: "Viope".to_string(), industry: "Education".to_string() };

    fn print_summary(info: &impl Summary) {
        info.summarize();
    }
    print_summary(&person);
    print_summary(&company);
}
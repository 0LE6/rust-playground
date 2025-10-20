// https://doc.rust-lang.org/book/ch13-00-functional-features.html#functional-language-features-iterators-and-closures

// Functional Language Features: Iterators and Closures

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

fn main() {
    println!("Hello from Rust book!"); 
}

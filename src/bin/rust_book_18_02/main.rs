// Using Trait Object to Abstract
// over Shared Behavior

mod lib;
use lib::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code
    }
}

fn main() {
        
}

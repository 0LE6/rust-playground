// 55: Hash maps in Rust

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Rusty"), 69);
    scores.insert(String::from("Frosty"), 75);
    dbg!(scores);
}

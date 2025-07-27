// 56: Hash maps in Rust (2)

use std::collections::HashMap;

fn main() {
    let mut items: HashMap<String, i32> = HashMap::new();
    items.insert(String::from("Cup"), 10);
    dbg!(&items);

    items.insert(String::from("Cup"), 25);
    dbg!(&items);
}

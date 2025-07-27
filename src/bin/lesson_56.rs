// 56: Hash maps in Rust (2)

use std::collections::HashMap;

fn main() {
    let mut items: HashMap<String, i32> = HashMap::new();
    items.insert(String::from("Cup"), 10);
    dbg!(&items);

    // items.insert(String::from("Cup"), 25);
    // dbg!(&items);
    
    // if key doens't exist
    // entry + or_insert
    items.entry(String::from("Cup")).or_insert(20); // returns mut ref to value of Cup (10)
    items.entry(String::from("Spoon")).or_insert(20);
    dbg!(items);
}

// 56: Hash maps in Rust (2)

use std::collections::HashMap;

fn main() {
    let mut items: HashMap<String, i32> = HashMap::new();
    items.insert(String::from("Cup"), 10);
    let i = items.entry("Cane".to_string()).or_insert(68);
    dbg!(&items); // Cane: 69, Cup: 10

    // items.insert(String::from("Cup"), 25);
    // dbg!(&items);
    
    // if key doens't exist
    // entry + or_insert
    // items.entry(String::from("Cup")).or_insert(20); // returns mut ref to value of Cup (10)
    // items.entry(String::from("Spoon")).or_insert(20);
    // dbg!(items);

    let txt = "Let's get Rusty with Rusty so let's get it";
    let mut map = HashMap::new();

    for word in txt.split_whitespace()  {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count += 1
    }

    dbg!(map);

}

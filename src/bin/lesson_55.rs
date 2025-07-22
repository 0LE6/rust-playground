// 55: Hash maps in Rust

use std::collections::HashMap;

fn main() {
    // creation
    let mut scores = HashMap::new();
    scores.insert(String::from("Rusty"), 69);
    scores.insert(String::from("Frosty"), 75);
    // dbg!(scores);
    
    // access
    // let score = scores
    //     .get("Rusty")
    //     .copied() // copy to not get just a &i32 ref (only if it implements Copy trait)
    //     .unwrap_or(0); // unwrap it with 0 if it doesn't exist
    // 
    // dbg!(score);

    // iterate
    for (k, v) in &scores {
        println!("{k} : {v} points"); 
    }

}

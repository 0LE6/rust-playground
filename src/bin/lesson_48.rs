// 48: When to panic! vs. when to use Result in Rust

// --- 3rd part ---
fn main() {
    
}

/*
//--- 2nd part ---
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "helloooooooooooooo"
        .parse()
        .expect("Hardcoded IP is invalid!");

    dbg!(home);
}
*/

/*
// --- 1st part ---
fn main() {
    let word = "Rusty";
    let first_char = get_first_char(word);

    dbg!(first_char);
}

fn get_first_char(string: &str) -> char {
    if string.is_empty() {
        panic!("String is empty, lmao!");
    }

    string.chars().next().unwrap()
}
*/

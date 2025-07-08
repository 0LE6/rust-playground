// 48: When to panic! vs. when to use Result in Rust

// --- 4th part ---

fn main() {
    let guess = Guess::new(500);
    println!("Your guess -> {}", guess.value());
}

struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 & 100, got {value}");
        }
        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}
/*
// --- 3rd part ---
fn main() {
    match call_api() {
        Ok(response) => println!("API response {}", response),
        Err(e) => println!("Error {}", e)
    } 
}

fn call_api() -> Result<String, String> {
    Err("API limit reached!".to_string())
}
*/

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

// 54: Indexing strings in Rust

fn main() {
    let greeting = String::from("Hola"); // vect storing str of 4 bytes
    let len = greeting.len(); // return len in bytes
    dbg!(len);

    // if it contains special chars
    let spec = String::from("Holá");
    let spec_len = spec.len(); // 5 bytes
    dbg!(spec_len);




}

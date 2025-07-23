// 26: cloning in Rust

fn main() {
    let mut var = String::from("Rusty"); // Rusty is released from memory
    var = String::from("Frosty"); // and its Frosty takes its place

    print!("Hello, {var}");
        
}

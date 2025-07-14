// 52: Strings are complicated in Rust?

fn main() {
    let name: &str = "Rusty";
    dbg!(name);

    let s = String::new();
    dbg!(s);

    let text = "Hello, Rusty!".to_string();
    dbg!(text);
}

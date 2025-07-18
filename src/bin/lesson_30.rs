// 30: Dangling references in Rust

fn main() {
    let r = get_reference();
    dbg!(r);
}

fn get_reference() -> String {
    let text = String::from("Rusty");
    text
}

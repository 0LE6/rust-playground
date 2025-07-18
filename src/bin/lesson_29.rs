// 29: Mutable references in Rust

fn main() {
    let mut text = String::from("Rusty");
    modify(&mut text);
    dbg!(text);
}

fn modify(text: &mut String) {
    text.push_str(" is cool!");
}

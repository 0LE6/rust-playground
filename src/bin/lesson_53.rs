// 53: format! in Rust

fn main() {
    let mut text = String::from("Rusty");
    text.push_str(" likes coding!"); // &'static str
    dbg!(text);

    let mut text_2 = String::from("Frostie");
    let ending = " likes cereals!"; // &'static str
    text_2.push_str(ending);
    dbg!(text_2);
    dbg!(ending);
}

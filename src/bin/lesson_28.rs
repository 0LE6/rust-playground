// 28: References in Rust

fn main() {
    let txt = String::from("Rusty");
    let len = get_length(&txt);

    println!("The lenght of {txt}: {len}");

    // dbg!(txt, len); // we took ownership, use better a ref 
    dbg!(&txt, &len);
}

fn get_length(txt: &str) -> usize {
    txt.chars().count()
}

// 36: Structs provide clarity in Rust

fn main() {
    // let rect = (20, 30); // emulating w & h
    let rect = Rectangle { width: 20, height: 30 };
    dbg!(get_area(&rect));
    print!("The rect w/ print {:?}", rect);
    dbg!(rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn get_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height 
}

// fn get_area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

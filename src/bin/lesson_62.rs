// 62: generic structs in Rust

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 3 };
    
    dbg!(p1, p2);
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

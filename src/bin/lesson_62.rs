// 62: generic structs in Rust
use num::Num;

fn main() {
    let p1 = Point { x: 1, y: 2 }; // allowed
    let p2 = Point { x: 1.5, y: 2.8 }; // allowed
    // let p3 = Point { x: true, y: false }; // not allowed
    // by using the generic T of Num value
    
    dbg!(p1);
}

#[derive(Debug)]
struct Point<T: Num> { // could also be <T, U>
    x: T,
    y: T
}

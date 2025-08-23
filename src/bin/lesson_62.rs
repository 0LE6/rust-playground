// 62: generic structs in Rust
use num::Num;

fn main() {
    let p1 = Point { x: 1, y: 2 }; // allowed
    let p2 = Point { x: 1.5, y: 2.8 }; // allowed
    // let p3 = Point { x: true, y: false }; // not allowed
    // by using the generic T of Num value
    // 1. the trait bound `bool: Num` is not satisfied
    // the following other types implement trait `Num`:
    //   BigInt
    //   BigUint
    //   Complex<T>
    //   Ratio<T>
    //   Wrapping<T>
    //   f32
    //   f64
    //   i128
    // and 11 others
    
    // dbg!(p1);
    example_2_test();
}

fn example_2_test() {
    let conn = true;
    let result = if conn { Some("Connected!") } else { None };

    dbg!(result);
}

// Example 2: generic enums
enum Option<T> {
    Some(T),
    None
}

// Example 1: generic structs
#[derive(Debug)]
struct Point<T: Num> { // could also be <T, U>
    x: T,
    y: T
}

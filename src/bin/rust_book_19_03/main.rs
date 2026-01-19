// Pattern Syntax

fn main() {
    // Matching Named Variables
    // let x = Some(5);
    // let y = 10;
    //
    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(y) => println!("Matched, y = {y}"),
    //     _ => println!("Default case, x = {x:?}"),
    // }
    //
    // println!("at the end: x = {x:?}, y = {y}");

    // Matching Multiple Patterns
    // let x = 1;
    //
    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // Matching Ranges of Values with ..=
    // let x = 5;
    //
    // match x {
    //     1..=5 => println!("one through five"),
    //     _ => println!("something else"),
    // }

    // example using ranges of char Values
    // let x = 'c';
    //
    // match x {
    //     'a'..='j' => println!("early ASCII letter"),
    //     'k'..='z' => println!("late ASCII letter"),
    //     _ => println!("something else"),
    // }

    // Destructuring to Break Apart Values
    let p = Point { x: 10, y: 7};

    // let Point { x: a, y: b } = p;
    //
    // assert_eq!(0, a); // FALSE
    // assert_eq!(7, b); // TRUE

    let Point { x, y } = p;
    assert_eq!(0, x); // FALSE
    assert_eq!(7, y); // TRUE
}

struct Point {
    x: i32,
    y: i32,
}

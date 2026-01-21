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
    // let p = Point { x: 10, y: 7};

    // let Point { x: a, y: b } = p;
    //
    // assert_eq!(0, a); // FALSE
    // assert_eq!(7, b); // TRUE

    // let Point { x, y } = p;
    // assert_eq!(0, x); // FALSE
    // assert_eq!(7, y); // TRUE

    // let p = Point { x: 0, y: 7};
    //
    // match p {
    //     Point { x, y: 0 } => {
    //         println!("On the x axis at {x}");
    //     },
    //     Point { x: 0, y } => {
    //         println!("On the y axis at {y}");
    //     },
    //     Point { x, y } => {
    //         println!("On neither axis: ({x}, {y})");
    //     },
    // }

    // let msg = Message::ChangeColor(0, 160, 255);
    //
    // match msg {
    //     Message::Quit => {
    //         println!(
    //             "Quit variant has no data to destruct"
    //         );
    //     }
    //     Message::Move { x, y } => {
    //         println!(
    //             "Move in the x dir {x} & y dir {y}"
    //         );
    //     }
    //     Message::Write(text) => {
    //         println!("Text message: {text}");
    //     }
    //     Message::ChangeColor(r, g, b) => {
    //         println!(
    //             "Change color to R {r}, G {g} & B {b}"
    //         );
    //     }
    // }

    // 19-16
    // let msg = Message::ChangeColor(
    //     Color::Hsv(0, 160, 255)
    // );

    let msg = Message::ChangeColor(
        Color::Rgb(69, 100, 220)
    );


    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change color to R {r}, G {g} & B {b}"
            );
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change color to H {h}, S {s} & V {v}"
            );
        }
        _ => (),
    }

}

enum Color {
    Rgb(i32, i32, i32),    
    Hsv(i32, i32, i32),    
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point {
    x: i32,
    y: i32,
}

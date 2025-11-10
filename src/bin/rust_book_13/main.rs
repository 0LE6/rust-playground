// https://doc.rust-lang.org/book/ch13-00-functional-features.html#functional-language-features-iterators-and-closures

use std::{thread, time::Duration};

// Functional Language Features: Iterators and Closures
// https://doc.rust-lang.org/book/ch13-01-closures.html#listing-13-1
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // closure example part
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );


    //https://doc.rust-lang.org/book/ch13-01-closures.html#closure-type-inference-and-annotation
    // assign closure to expensive_closure variable
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };
    
    let x = expensive_closure(3);
    dbg!(x);

    // another example (4 equivalents)    
    fn add_one_v1(x: u32) -> u32  { x + 1 }
    let add_one_v2 = |x: u32| -> u32  { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    
    let n: u32 = 68;

    dbg!(
        add_one_v1(n), // 69
        add_one_v2(n), // 69
        add_one_v3(n), // 69
        add_one_v4(n), // 69
    );

    // example to fail
    let example_closure = |x| x;

    // Attempting to call a closure whose types are inferred
    // with two different types.
    let s = example_closure(String::from("hello"));

    // the first time we call the closure it gets the String
    // type and fails when we try reasign it to another.
    // let n = example_closure(5);
    
    // https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership
    
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    //
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // https://doc.rust-lang.org/book/ch13-01-closures.html#listing-13-5
    // let mut only_borrows = || list.push(69);

    // println!("Before calling the closure: {list:?}");
    // only_borrows();
    // println!("After calling the closure: {list:?}");
}

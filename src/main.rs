// use std::str;
use rand::Rng;

// 42: "match" like a pro in Rust

fn main() {
    // let user = Some("Rusty");
    // let result = user_exists(user);
    // println!("result = {:?}", result)
    let roll = roll_dice();
    board_event(roll);
}

fn roll_dice() -> u8 {
    let mut rng = rand::rng();
    let roll = rng.random_range(1..=6);
    println!("Rusty rolled a: {roll}");
    
    roll
}

fn board_event(roll: u8) {
    match roll {
        1 => println!("Rusty goes to jail!"),
        2 => println!("Rusty wins the lottery!"),
        // _ => println!("Rusty does nothing :)"),
        _ => (),
    }
}

// enum Grade {
//     A,
//     B,
//     C,
// }
//
// fn grade_to_score(grade: Grade) -> u8 {
//     match grade {
//         Grade::A => 100,
//         Grade::B => 90,
//         Grade::C => 80,
//     }
// }

// fn user_exists(user: Option<&str>) -> bool {
//     match user {
//         None => {
//             println!("Pls, insert a username!");
//             false
//         }
//         Some(user) => {
//             println!("Looking for user...");
//             println!("'{user}' found! :D");
//             true
//         }
//     }
// }

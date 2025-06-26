// 43 - "if let" is pretty hype in Rust 

fn main() {
    // println!("Hello from test.rs! :)")
    // let user_input: Option<i32> = Some(10);
    // set_brightness(user_input);
    // set_brightness(None);

    let coin: Crypto = Crypto::Btc(100);

    if let Crypto::Btc(amount) = coin {
        println!("You're rich mate!");
    } else {
        println!("Other crypto coin...");
    }

    // with match
    match coin {
        Crypto::Btc(amount) => println!("Richie rich!"),
        _ => ()
    }
}


enum Crypto {
    Btc(i32),
    Eth(i32)
}

// fn set_brightness(brightness: Option<i32>) {
//     if let Some(value) = brightness {
//          println!("The bgrth set to {value}%");
//     } else {
//         println!("No brgth set!")
//     }
//
//
//     // match brightness {
//     //     Some(value) => println!("The bgrth set to {value}%"),
//     //     _ => ()
//     // }
// }

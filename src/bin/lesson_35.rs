// 35: Tuple structs are pretty cool in Rust

// fn main() {
//     let blue = Color(0, 0, 255);
//     let date = Date(11, 07, 2025);
//
//     display_date(&date);
//
//     let Date(day, month, year) = date;
//     dbg!(day, month, year);
// }
//
// struct Color(u16, u16, u16);
// struct Date(u16, u16, u16);
//
// fn display_date(date: &Date) {
//     println!("Date -> {}/{}/{}", date.0, date.1, date.2);
// }

use std::str::FromStr;


fn main() {
    let user = User { 
        id: 0, 
        username: String::from("Pepe"), 
        email: String::from("pepe@pp.com") 
    };
    
    dbg!(user.username);
}

#[derive(Debug)]
struct  User {
    id: i32,
    username: String, 
    email: String,
}

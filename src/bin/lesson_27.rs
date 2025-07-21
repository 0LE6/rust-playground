// 27: Ownership in Rust

fn main() {
    
    // 2)
    let s1 = create_string();
    let s2 = create_string();
    dbg!(s1, s2); // dbg macro took Ownership

    // 1)
    // let name = String::from("Rusty");
    // greet(name); // fn takes Ownership and the value ain't valid anymore 
    // 
    // let n = 69;
    // display_number(n);
    // println!("Second attempt to use number: {n}"); // this will work 'cause i32 uses copy trait
}

fn create_string() -> String {
    String::from("Rusty") 
}

// 1)
// fn greet(name: String) {
//     println!("Hello, {name}!");
// }
//
// fn display_number(n: i32) {
//     println!("The number is {n}");
// }

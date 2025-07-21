// 27: Ownership in Rust

fn main() {
    let name = String::from("Rusty");
    greet(name); // fn takes Ownership and the value ain't valid anymore 
    
    let n = 69;
    display_number(n);
    println!("Second attempt to use number: {n}"); // this will work 'cause i32 uses copy trait
}

fn greet(name: String) {
    println!("Hello, {name}!");
}

fn display_number(n: i32) {
    println!("The number is {n}");
}

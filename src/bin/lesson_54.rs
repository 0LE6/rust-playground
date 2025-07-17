// 54: Indexing strings in Rust

fn main() {
    // 1)
    // let greeting = String::from("Hola"); // vec stors str of 4 bytes
    // let len = greeting.len(); // return len in bytes
    // dbg!(len);

    // 2)
    // if it contains special chars
    // let spec = String::from("Holá");
    // let spec_len = spec.len(); // 5 bytes
    // dbg!(spec_len);

    // 3)
    // let greeting = String::from("привет");
    // let unicode_scalars: Vec<char> = greeting.chars().collect();
    // println!("unicode_scalars = {:?}", unicode_scalars);
    
    // 4)
    let cluster = Vec::from(["न", "म", "स्","ते"]);
    println!("cluster = {:?}", cluster); 
    // cluster = ["न", "म", "स\u{94d}", "त\u{947}"]
    
    

}

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
    // let cluster = Vec::from(["न", "म", "स्","ते"]);
    // println!("cluster = {:?}", cluster); 
    // cluster = ["न", "म", "स\u{94d}", "त\u{947}"]
    
    // 5) - slicin
    // let greeting = String::from("Holá");

    // byte index 4 is not a char boundary; 
    // it is inside 'á' (bytes 3..5) of `Holá`
    // let s = &greeting[3..4];

    // let s = &greeting[3..5]; // 'á' 
    // dbg!(s);

    // 6) 
    let word = "ラーメン";
   
    // you  must iterate over chars or bytes
    for c in word.chars() {
        dbg!(c);
    }

    for c in word.bytes() {
        dbg!(c);
    }
}

// 26: cloning in Rust

fn main() {
    // 1)
    // let mut var = String::from("Rusty"); // Rusty is released from memory
    // var = String::from("Frosty"); // and its Frosty takes its place
    //
    // print!("Hello, {var}");
   
    // 2) - another example
    let var = String::from("Rusty");
    let var2 = var; // takes the value and 'var' is no longer exists
    // dbg!(var); // value moved and doesn't exist 
    dbg!(var2); // now the value its in the 'var2'
    
}

// 26: cloning in Rust

fn main() {
    // 1)
    // let mut var = String::from("Rusty"); // Rusty is released from memory
    // var = String::from("Frosty"); // and its Frosty takes its place
    //
    // print!("Hello, {var}");
   
    // 2) - another example
    // let var = String::from("Rusty");
    // let var2 = var; // takes the value and 'var' is no longer exists
    // // dbg!(var); // value moved and doesn't exist 
    // dbg!(var2); // now the value its in the 'var2'
   
    // 3)
    let name = String::from("Frosty");
    let name_copy = name.clone();

    dbg!(name, name_copy);

    // but we don't need to do it with Integers
    let n: i32 = 69;
    let n2 = n;

    // this works fine 'cause small types as 
    // Integers, Booleans, Floats, Chars, Tuples
    // (if contains some of mentioned ones) ... 
    // implements the Copy trait inside
    dbg!(n, n2);

}

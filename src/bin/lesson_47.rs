// 47: The Try operator (?) is amazing in Rust

// for the first testing
// use std::fs::{self, File};
// use std::io::{self, Read};

// fro the last one
use std::fs;
use std::io;

// fn get_data() -> Result<String, io::Error> {
//     // let mut result = String::new();
//     // File::open("result.txt")?.read_to_string(&mut result)?;
//     // Ok(result)
//    
//     fs::read_to_string("result.txt")
// }

fn last_char_of_first_line(text: &str) -> Option<char>{
   text.lines().next()?.chars().last() 
}

fn main() {
   let c = last_char_of_first_line("Hey, you're Rusty\n or not..."); // las char of the line would be 'y' 
    dbg!(c);

    let empty = last_char_of_first_line("");
    dbg!(empty); // here we'll get a None

    // let result = get_data();
    // dbg!(result); // we'll obtain an error 'cause there's no file  
}

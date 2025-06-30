// 46: Propagating error in Rust

use std::fs::File;
use std::io::{self, Read};

fn get_data() -> Result<String, io::Error> {
    let data_file_result = File::open("data.txt");

    let mut data_file = match data_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut data = String::new();

    match data_file.read_to_string(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e),
    }
}

fn main() {

    let data = get_data();
    dbg!(data); // we'll obtain an error 'cause there's no file
    // let file = File::open("secret.txt").unwrap();
    // dbg!(file);

    // let file = File::open("file.txt")
    //     .expect("File is missing! D:");
    // dbg!(file); 
    
    
}

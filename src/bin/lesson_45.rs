// 45: Handling multiple errors with "Result" in Rust

use std::{fs::File, io::{ErrorKind, Read}};

fn main() { 
    let path: &'static str = "sample.txt"; 
    // let path: &'static str = "secret.png";
    let txt = File::open(path);

    // match txt {
    //     Ok(mut file) => {
    //         let mut contents = String::new();
    //         let _ = file.read_to_string(&mut contents);
    //         println!("File loaded: {}", contents);
    //     }
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // }
    
    let txt = match txt {
       Ok(file) => {
            println!("Loaded '{path}' successfully!");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => {
                    println!("New file created! ({path})");
                    file
                }
                Err(error) => panic!("Problem creating new file {error}")
            }
            _ => {
                panic!("Problem opening the file: {error}")
            }
        }
    };
}



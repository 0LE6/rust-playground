// let's try to do a mini grep example
// extracting a word from a textfile

use::std::env::args;
use std::fs;

fn main() {

    let args: Vec<String> = args().collect();
    // let query = &args[1];
    // let file = &args[2];

    // dbg!(&query, &file);

    // let content = fs::read_to_string(&file)
        // .expect("::: There's no such file! :::'");
    // let config = parse_config(&args);
    
    let config = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let content = fs::read_to_string(config.file_path)
        .expect("::: There's no such file! :::'");

    dbg!(content);

    // dbg!(parse_config(&args));

}

struct Config {
    query: String,
    file_path: String,
}

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#creating-a-constructor-for-config

// Returning a Result Instead of Calling panic!
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#returning-a-result-instead-of-calling-panic
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, mate!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query , file_path })
    }
}


// impl Config {
//     fn new(args: &[String]) -> Config {
//         // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#improving-the-error-message
//         if args.len() < 3 {
//             panic!(":: Not enough arguments, mate!")
//         }
//
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//
//         Config { query , file_path }
//     }
// }


// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#grouping-configuration-values
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//
//     Config { query, file_path }
// }

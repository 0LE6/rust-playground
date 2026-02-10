use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind(
        "127.0.0.1:6969"
    ).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_connection(stream);
        // println!("Connection established!");
    }
}

fn handle_connection(
    mut stream: TcpStream
) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    // let response = b"HTTP/1.1 200 OK\r\n\r\n"; 

    stream.write_all(response.as_bytes()).unwrap();
    // stream.write_all(response).unwrap(); 
    // directly receiv bytes
}

















// ---------------------------------------------------

// // let's try to do a mini grep example
// // extracting a word from a textfile
//
// use::std::env::args;
// use std::{env, error::Error, fs, process};
//
// use rust_playground::{search, search_case_insensitive};
// use::rust_playground::Config; // lib.rs
//
// pub struct Configuration {
//     pub query: String,
//     pub file_path: String,
//     pub ignore_case: bool,
// }
//
// impl Configuration {
//     fn build(args: &[String]) -> Result<Configuration, &'static str> {
//         if args.len() < 3 {
//             return Err("Not enough arguments!");
//         }
//
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//
//         let ignore_case = env::var("IGNORE_CASE").is_ok();
//
//         Ok(Configuration { 
//             query, 
//             file_path, 
//             ignore_case 
//         })
//     }
// }
//
// fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
//
//     let results = if config.ignore_case {
//         search_case_insensitive(&config.query, &contents)
//     } else {
//         search(&config.query, &contents)
//     };
//
//     for line in results {
//         println!("{line}");
//     }
//
//     Ok(())
// }
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let config = Configuration::build(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });
//
//     if let Err(e) = run(config) {
//         println!("Application error: {e}");
//         process::exit(1);
//     }
// }
//
// // fn main() {
// //
// //     //Splitting Code into a Library Crate
// //     
// //     let args: Vec<String> = args().collect();
// //
// //     // using or lib.rs
// //     // let config = Config::build(&args).unwrap_or_else(|err| {
// //     //     println!("Problem parsing arguments: {err}");
// //     //     process::exit(1);
// //     // });
// //     //
// //     // if let Err(e) = rust_playground::run(config) {
// //     //     println!("Application error: {e}");
// //     //     process::exit(1);
// //     // }
// //
// //     let uwu = "\
// //         qwerty
// //         asdf
// //         pepito
// //         rust";
// //
// //     let owo = rust_playground::search("pepito", uwu);    
// //
// //     // let args: Vec<String> = args().collect();
// //     // let query = &args[1];
// //     // let file = &args[2];
// //
// //     // dbg!(&query, &file);
// //
// //     // let content = fs::read_to_string(&file)
// //         // .expect("::: There's no such file! :::'");
// //     // let config = parse_config(&args);
// //    
// //     // 1st from ---------------------------------
// //     // let config = match Config::new(&args) {
// //     //     Ok(c) => c,
// //     //     Err(e) => {
// //     //         eprintln!("Error: {}", e);
// //     //         std::process::exit(1);
// //     //     }
// //     // };
// //     
// //     // 2nd form ---------------------------------
// //     // let config = Config::build(&args).unwrap_or_else(
// //     //     |err| {
// //     //         println!("Error parsing args {err}");
// //     //         process::exit(1);
// //     //     }
// //     // );
// //
// //     // 3rd form ----------------------------------
// //     // run(config);
// //
// //     // let content = fs::read_to_string(config.file_path)
// //     //     .expect("::: There's no such file! :::'");
// //
// //     // dbg!(content);
// //
// //     // dbg!(parse_config(&args));
// //     
// //     // Handling Errors Returned from run in main
// //     // if let Err(e) = run(config) {
// //     //     println!("Application error: {e}");
// //     //     process::exit(1);
// //     // }
// // }
//
//
// // impl Config {
// //     fn new(args: &[String]) -> Config {
// //         // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#improving-the-error-message
// //         if args.len() < 3 {
// //             panic!(":: Not enough arguments, mate!")
// //         }
// //
// //         let query = args[1].clone();
// //         let file_path = args[2].clone();
// //
// //         Config { query , file_path }
// //     }
// // }
//
//
// // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#grouping-configuration-values
// // fn parse_config(args: &[String]) -> Config {
// //     let query = args[1].clone();
// //     let file_path = args[2].clone();
// //
// //     Config { query, file_path }
// // }


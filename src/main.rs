// let's try to do a mini grep example
// extracting a word from a textfile

use::std::env::args;
use std::fs;

fn main() {

    let args: Vec<String> = args().collect();
    let query = &args[1];
    let file = &args[2];

    // dbg!(&query, &file);

    let content = fs::read_to_string(&file)
        .expect("::: There's no such file! :::'");

    // dbg!(content);

    dbg!(parse_config(&args));

}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

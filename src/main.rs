// let's try to do a mini grep example
// extracting a word from a textfile

use::std::env::args;

fn main() {

    let args: Vec<String> = args().collect();
    let query = &args[1];
    let file = &args[2];

    dbg!(query, file);

}

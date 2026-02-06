// Macros
mod lib;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // let mut vec = vect![1, 2, 3];
    // println!("vec: {:?}", vec);

    Pancakes::hello_macro();

}

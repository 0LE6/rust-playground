// Macros
mod lib;

use lib::HelloMacro;
// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, MAcro! My name is Pancakes!");
    }
}

fn main() {
    // let mut vec = vect![1, 2, 3];
    // println!("vec: {:?}", vec);

    Pancakes::hello_macro();

}

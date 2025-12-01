// Using Box<T> to Point to Data on the Heap
//
// * Boxes allow you to store data on the heap rather than the stack.
// 
// Usage situations:
//
// *    When you have a type whose size can’t be known at compile 
//      time and you want to use a value of that type in a context 
//      that requires an exact size
//
// *    When you have a large amount of data and you want to transfer 
//      ownership but ensure the data won’t be copied when you do so
//
// *    When you want to own a value and you care only that 
//      it’s a type that implements a particular trait rather than 
//      being of a specific type
use crate::List::{Cons, Nil};

fn main() {
    // let b = Box::new(69);
    // println!("b = {b}");

    let list = Cons(
        1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))
    );
}

// It will give an error because
// this type “has infinite size.”

// Compiler advice:
// * help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) 
// * to break the cycle
enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    // The Box<T> will point to the next List 
    // value that will be on the heap rather 
    // than inside the Cons variant.
    Nil,
}

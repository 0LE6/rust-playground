// Rc<T>, the Reference Counted Smart Pointer 

use std::rc::Rc;
use crate::List::{Nil, Cons};

fn main() {
    let a = Rc::new(
        Cons(
            5, Rc::new(
                Cons(
                    10, Rc::new(Nil)
                )
            )
        )
    );
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // Rc::clone doesn’t make a deep copy
    // of all the data like most types’ 
    // implementations of clone do, it only 
    // increments the reference count,
    // which doesn’t take much time.
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

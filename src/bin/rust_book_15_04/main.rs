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
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

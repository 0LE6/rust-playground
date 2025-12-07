// Rc<T>, the Reference Counted Smart Pointer 
use crate::List::{Nil, Cons};

fn main() {
    let a = Cons(
        5, 
        Box::new(Cons(10, Box::new(Nil)))
    );
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

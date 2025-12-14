// Reference Cycles Can Leak Memory

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(
        &self
    ) -> Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    } 
}

fn main() {
    
}

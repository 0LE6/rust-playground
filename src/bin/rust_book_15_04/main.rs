// Rc<T>, the Reference Counted Smart Pointer 

use std::rc::Rc;
use crate::List::{Nil, Cons};

fn main() {
    // let a = Rc::new(
    //     Cons(
    //         5, Rc::new(
    //             Cons(
    //                 10, Rc::new(Nil)
    //             )
    //         )
    //     )
    // );
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    // Rc::clone doesn’t make a deep copy
    // of all the data like most types’ 
    // implementations of clone do, it only 
    // increments the reference count,
    // which doesn’t take much time.

    // Cloning an Rc<T> Increases the Reference Count 
    let a = Rc::new(
        Cons(
            5, Rc::new(
                Cons(
                    10, Rc::new(Nil)
                )
            )
        )
    );
    println!(
        "count after creating a = {}",
        Rc::strong_count(&a)
    );

    let b = Cons(3, Rc::clone(&a));
    println!(
        "count after creating b = {}",
        Rc::strong_count(&a)
    );
    {
        let c = Cons(4, Rc::clone(&a));
        println!(
            "count after creating c = {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );

    // Using Rc<T> allows a single value 
    // to have multiple owners, and the 
    // count ensures that the value remains 
    // valid as long as any of the owners still exist.
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

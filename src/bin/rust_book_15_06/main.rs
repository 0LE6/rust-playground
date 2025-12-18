/// Reference Cycles Can Leak Memory

use std::{cell::RefCell, rc::{Rc, Weak}};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(
        &self
    ) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    } 
}

fn main() {
    // let a = Rc::new(
    //     Cons(5, RefCell::new(Rc::new(Nil)))
    // );
    //
    // println!(
    //     "a initial rc count = {}",
    //     Rc::strong_count(&a)
    // );
    // println!(
    //     "a next item = {:?}",
    //     a.tail()
    // );
    //
    // let b = Rc::new(
    //     Cons(10, RefCell::new(Rc::clone(&a)))
    // );
    //
    // println!(
    //     "a rc count after b creation = {}",
    //     Rc::strong_count(&a)
    // );
    // println!(
    //     "b initial rc count = {}",
    //     Rc::strong_count(&b)
    // );
    // println!(
    //     "b next item = {:?}",
    //     b.tail()
    // );
    //
    // if let Some(link) = a.tail() {
    // println!(
    //     "1 ----- = {}",
    //     Rc::strong_count(&a)
    // );
    //     *link.borrow_mut() = Rc::clone(&b);
    // println!(
    //     "2 ----- = {}",
    //     Rc::strong_count(&a)
    // );
    // }
    //
    // println!(
    //     "b rc count after changing a = {}",
    //     Rc::strong_count(&b)
    // );
    // println!(
    //     "a rc count after changing a = {}", 
    //     Rc::strong_count(&a)
    // );

    // Uncomment the next line to see 
    // that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());


    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(
            vec![Rc::clone(&leaf)]
        ),
    });

    *leaf.parent.borrow_mut() = 
        Rc::downgrade(&branch); 
    // to create Weak<Node> ref to branch

    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );
    // the Node in leaf now has two owners: 
    // leaf and branch


}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// Treating Smart Pointers Like 
// Regular References with "Deref"

use std::ops::Deref;

fn main() {
    let string = MyBox::new(
        String::from("Rusty")  
    );
    hello(&string);
}

// Implicit Deref Coercions with Functions and Methods
fn hello(name: &str) {
    println!("Hello, {name} !");
}


// Defining Our Own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementing the Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    // Rust substitutes the * operator
    // with a call to the deref method
    fn deref(&self) -> &Self::Target {
        &self.0 // 0 is T, the 1st & only value
    }
}

#[test]
fn test() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

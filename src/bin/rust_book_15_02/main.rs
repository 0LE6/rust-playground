// Treating Smart Pointers Like 
// Regular References with "Deref"

fn main() {
    let x = 5;
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining Our Own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

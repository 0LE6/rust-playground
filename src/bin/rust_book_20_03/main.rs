// Advanced Types

fn main() {

    type Thunk = 
        Box<dyn Fn() + Send + 'static>;
    
    let f: Thunk = 
        Box::new(|| println!("hi"));

    fn take_long_type(f: Thunk) {
        // something
    }

    fn returns_long_type() -> Thunk {
        Box::new(|| ())        
    }

    fn generica<T: ?Sized>(t: &T) {
        // something        
    }
}

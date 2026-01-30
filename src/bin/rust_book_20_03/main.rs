// Advanced Types

fn main() {
    
    let f: Box<dyn Fn() + Send + 'static> = 
        Box::new(|| println!("hi"));

    fn take_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // something
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| ())        
    }
}

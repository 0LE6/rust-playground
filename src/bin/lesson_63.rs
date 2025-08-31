// 63: generics 2

use std::fmt::Display;

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 2.0 };
    
    // p1.i32_method();

    // example 1 
    // dbg!(p1.coordinates(), p2.coordinates());

    // example 2
    // let point_with_label = p2.label("Coordinates");
    // dbg!(point_with_label);

    print_this(43);
    print_this("Rusty Rust!");

}

// example 3 
fn print_this<T: Display>(arg: T) {
    println!("{}", arg); 
}

struct Point<T> {
    x: T,
    y: T
}

// impl Point<i32> {
//     fn i32_method(&self) {
//         println!("This only visible for i32!");
//     }
// }

// example 2
impl<T> Point<T> {
    fn label<L>(&self, label: L) -> (L, &T, &T) {
        (label, &self.x, &self.y)        
    }
}

// example 1
// impl<T> Point<T> {
//     fn coordinates(&self) -> (&T, &T) {
//         (&self.x, &self.y)        
//     }
// }

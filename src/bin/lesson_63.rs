// 63: generics 2

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 2.0 };
    
    // p1.i32_method();

    // example 1 
    // dbg!(p1.coordinates(), p2.coordinates());

    // example 2
    let point_with_label = p2.label("Coordinates");
    dbg!(point_with_label);
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

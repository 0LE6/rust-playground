// 63: generics 2

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 2.0 };
    
    dbg!(p1.coordinates(), p2.coordinates());
}

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn coordinates(&self) -> (&T, &T) {
        (&self.x, &self.y)        
    }
}

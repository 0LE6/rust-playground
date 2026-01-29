/* Advanced Trais */

use std::ops::Add;

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Millimeters( 2000) + Meters(2),
        Millimeters(4000)
    );

    /* In a fail situation

    assertion `left == right` failed
    left: Millimeters(4000)
    right: Millimeters(4001)
    */

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // *waving arms furiously*

}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking."); 
    }   
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Fly, you fools!");
    }   
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }   
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;    

    fn add(self, other: Point) -> Point {
        Point { 
            x: self.x + other.x,
            y: self.y + other.y, 
        }
    }
}

// struct Counter {
//     count: u32,
// }
//
// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0 }
//     }
// }
//
// impl Iterator for Counter {
//     type Item = u32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// an hypothetical definition
// of a trait using generics
// pub trait iterator<t> {
//     fn next(&mut self) ->option<t>;
// }

// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
// }

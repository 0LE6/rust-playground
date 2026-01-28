/* Advanced Trais */

fn main() {
    
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

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

// 37: Methods are useful in Rust

fn main() {
    println!("Rusty rust!");

    let mut rect = Rectangle { 
        width: 20, 
        height: 30,
    };

    // dbg!(rect.get_area());
    // rect.display();
    
    // both tuples indicates the same
    rect.sample_1();
    (&rect).sample_1();

    rect.sample_2();
    (&mut rect).sample_2();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn sample_1(&self) {}
    fn sample_2(&mut self) {}

    // fn is_valid(&self) -> bool  {
    //     self.width > 0 && self.height > 0
    // }
    //
    // fn display(&self) {
    //     if self.is_valid() {
    //         println!("Rectangle -> {} square pixels", self.get_area())
    //     } else {
    //         println!("The rectangle is invisible lol!")
    //     }
    // }
    //
    // fn get_area(&self) -> u32 {
    //     self.width * self.height
    // }
}

// 58

// 1)
// Crates
//
// rustc src/bin/lesson_58.rs 
// creates an executable with the name "lesson_58"
//
// if we execute that file it'll give us the print of "Lesson 58"

// 2)
// to add crates (i.e.: random generator)
// crates would be the similar to libraries in other langs

use rand::Rng;

fn roll() {
    let mut rng = rand::rng();
    let roll = rng.random_range(1..=6);
    print!("You rolled {roll}\n");
}

fn main() {
    // println!("Lesson 58");

    roll();
    roll();
    roll();
    roll();
}

// 3) packages
// cargo new <pkg_name>
// will create another folder that will contain 
// it own .toml to load crates, i.e. the rand we 
// added before
// https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html


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
//

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

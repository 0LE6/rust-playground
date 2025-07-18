// 29: Mutable references in Rust

fn main() {
    // 1)
    // let mut text = String::from("Rusty");
    // modify(&mut text);
    // dbg!(text);

    // 2)
    let mut text = String::from("Frosty");

    // this won't work
    // let r1 = &mut text;
    // let r2 = &mut text; // cannot borrow `text` as mutable more than once at a time
    // dbg!(r1, r2); 
    
    // this will
    // let r1 = &text;
    // let r2 = &text;
    // dbg!(r1, r2);
    //
    // let r3 = &mut text;
    // dbg!(r3);

    // 3)
    // let mut x = 10;
    // let y = &mut x;
    // x += 3;
    // // dbg!(y); //cannot use x cause it was mutably borrowed
    
   
    // correct representation of last example
    let mut x = 10;
    let y = &mut x;     // ← mutable borrow of `x`
    dbg!(y);            // ← using `y`, borrow ends after this line

    x += 3;             // ← OK: mutable borrow is over
    dbg!(x);            // ← now `x` is accessible again
}

// 1)
// fn modify(text: &mut String) {
//     text.push_str(" is cool!");
// }

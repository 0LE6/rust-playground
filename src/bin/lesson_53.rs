// 53: format! in Rust

fn main() {
    // 1)
    // let mut text = String::from("Rusty");
    // text.push_str(" likes coding!"); // &'static str
    // dbg!(text);
    //
    // let mut text_2 = String::from("Frostie");
    // let ending = " likes cereals!"; // &'static str
    // text_2.push_str(ending);
    // dbg!(text_2);
    // dbg!(ending);
    
    // let mut text = String::from("Hello");
    // text.push('!');
    // dbg!(text);
    
    // 2)
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("Rusty!");
    // let s3 = s1 + &s2; // s2 should be a ref
    // dbg!(s3);
    // and s1 will not longer exist 'cause the value was moved
        
    // 3) The proper way
    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = String::from("s3");

    let s4 = format!("{s1}--{s2}--{s3}"); // better way to do it
    // comparing to the 1) and 2) examples
    dbg!(s4);
    dbg!(s1, s2, s3); // and still can use the s1, s2 and s3 variables

}

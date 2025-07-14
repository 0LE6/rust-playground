// 32: &str is really cool in Rust

fn main() {
    // let sentence = String::from("Rusty is fast!");
    
    // let name = &sentence[..4]; // same as [0..4]
    // let speed = &sentence[9..]; // same as [9..14]
    // let full_string = &sentence[..];
    // let test = &sentence.chars().nth(3); // Some(t)
    // dbg!(name, speed, ful_string);

    // ------------------------------------------------------------
    // Important!
    // let name = String::from("José Miguel");
    // dbg!(&name[..4]); // byte index 4 is not a char boundary; it is inside 'é' (bytes 3..5) of `José Miguel`
    // dbg!(&name[..5]); // now we have full José string part
    // ------------------------------------------------------------

    // Example with String
    // let mut string = String::from("Holy wacamole!");
    // let first = get_first_word(&string);
    // dbg!(first);
    // string.clea(); // clean as the let is mut
    
    // Example with &str
    // let sentence = String::from("Rusty Rust!");
    // let sentence2 = "Blazingly fast!";
    //
    // let word = get_first_word(&sentence);
    // let word2 = get_first_word(sentence2);
    
    // Exampel with Arrays
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[2..];
    dbg!(slice);
        
}

fn get_first_word(sentence: &str) -> &str {
// fn get_first_word(sentence: &String) -> &str {
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
           return &sentence[..i]; 
        }
    }

    &sentence[..]
}


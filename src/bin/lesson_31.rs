// 31: Preparing for string slices in Rust 

fn main() {
    let elements = ['a', 'b', 'c'];

    // for element in elements.iter().enumerate() {
    //     println!("{:?}", element);
    // }
    
    let mut sentence = String::from("Rusty goes to the beach!");
    /* dbg!(get_first_word(&sentence)); */
    let first_word = get_first_word(&sentence);
    println!("first_word -> {:?}", first_word);

    sentence.clear();
    

}

fn get_first_word(sentence: &String) -> usize {
    let bytes = sentence.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    sentence.len()
}

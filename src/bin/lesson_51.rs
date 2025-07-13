// 51: Vectors are awesome in Rust

fn main() {
    // 1) start example
    // let v:Vec<i32> = Vec::new();
    // dbg!(v);

    // 2) basic example
    // let v = vec![1, 2, 3];
    // print!("v={:?}", v);
    // dbg!(v);

    // 3) adition example
    // let mut numbers = vec![0];
    //
    // numbers.push(69);
    // numbers.push(420);
    // numbers.push(3);
    // 
    // // dbg!(numbers); 
    // print!("v={:?} \n", numbers); 
    // numbers.pop();
    // print!("v={:?} \n", numbers); 

    // 4) referece example
    let mut numbers = vec![1, 2, 3];
    // numbers.push(4);

    let second = numbers[1];
    dbg!(second);
    // dbg!(&numbers);

    let third: &i32 = &numbers[2];
    dbg!(third);
  
    // if we pop the 3rd number it'll lead us to the None (_) 
    // otherwise it'll show uis the 3rd number
    numbers.pop();
    let third: Option<&i32> = numbers.get(2);
    match third {
        Some(third) => println!("The 3rd number -> {third}"),
        _ => println!("There's no 3rd element'"),
    }

}

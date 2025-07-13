// 51: Vectors are awesome in Rust

use std::f64::consts::PI;

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
    // let mut numbers = vec![1, 2, 3];
    // numbers.push(4);

    // let second = numbers[1];
    // dbg!(second);
    // dbg!(&numbers);

    // let third: &i32 = &numbers[2];
    // dbg!(third);
  
    // if we pop the 3rd number it'll lead us to the None
    // otherwise it'll show uis the 3rd number
    // numbers.pop();
    // let third: Option<&i32> = numbers.get(2);
    // match third {
        // Some(third) => println!("The 3rd number -> {third}"),
        // None => println!("There's no 3rd element'"),
    // }


    // let chars = vec!["Rusty", "Ferris"];
    // let pepe = chars.get(10);
    // dbg!(pepe); // it'll return None

    // let mut ns = vec![1, 2, 3, 4, 5];
    // ns.push(6); // like this we can 
    // let first = &ns[0]; // inmutable borrow
    // ns.push(6); // cannot fter the previous borrow 
    // println!("the 1st element -> {first}");

    // ref of a ref in a for
    // let ppl = vec!["Rusty", "Pepe", "Ferris"];
    // for p in &ppl {
    //     println!("{p}");
    // }

    // let mut nums = vec![1, 2, 3];
    // for n in &mut nums {
    //     *n += 10;
    // }
    //
    // println!("let mut nums -> {:?}", nums);

    // Last example
    let mut values = vec!{Value::Float(PI), Value::Int(42)};
    values.push(Value::Text(String::from("Rusty")));
   
    println!("values -> {:?}", values);
}

// better be on the top, but anyways (for last example)
#[derive(Debug)]
enum Value {
    Int(i32),
    Float(f64),
    Text(String),
}


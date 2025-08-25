// 19: loop

fn main() {
    // example break
    // let mut num = 10;
    // loop {
    //     println!("Let's get rusty by {num}");
    //     num -= 1;
    //   
    //     if num == 0 {
    //         println!("Loop is finished!");
    //         break;
    //     }
    // }

    // example return
    let mut num = 0;
    let result = loop {
        println!("Let's get rusty by {num}");
        num += 1;
      
        if num == 5 {
            break "SUCCESS!";
        }
    };

    dbg!(result);
}

// 19: loop

fn main() {
    let mut num = 10;
    loop {
        println!("Let's get rusty by {num}");
        num -= 1;
      
        if num == 0 {
            println!("Loop is finished!");
            break;
        }
    }
}

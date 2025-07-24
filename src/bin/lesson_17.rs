// 17: "Let if" in Rust

fn main() {
    // let n = 11;
    // let odd_even = if n % 2 == 0 {"Even"} else {"Odd"};
    //
    // dbg!(odd_even);
    
    let is_connected = false;
    let result = if is_connected {0} else {-1};
    
    dbg!(result);
}

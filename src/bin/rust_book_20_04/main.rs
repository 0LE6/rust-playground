// Advanced Functions and Closures

fn main() {
    // let ans = do_twice(add_one, 5);   
    //
    // println!("The answer is: {ans}");

    let list_of_nums: Vec<i32> = Vec::from([1, 2, 3]);
    // let list_of_nums = vec![1, 2, 3];
    let list_of_strs: Vec<String> = list_of_nums
        .iter()
        .map(|n| n.to_string())
        .collect();

    println!("nums {:?}", list_of_nums);
    println!("strs {:?}", list_of_strs);


}

// fn add_one(x: i32) -> i32 {
//     x + 1    
// }
//
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

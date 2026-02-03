// Advanced Functions and Closures

fn main() {
    // let ans = do_twice(add_one, 5);   
    //
    // println!("The answer is: {ans}");

    // let list_of_nums: Vec<i32> = Vec::from([1, 2, 3]);
    // // let list_of_nums = vec![1, 2, 3];
    // let list_of_strs: Vec<String> = list_of_nums
    //     .iter()
    //     // .map(|n| n.to_string())
    //     .map(ToString::to_string)
    //     // Fn(&T) -> U
    //     // == |n: &T| n.to_string()
    //     .collect();
    //
    // println!("nums {:?}", list_of_nums);
    // println!("strs {:?}", list_of_strs);

    // #[derive(Debug)]
    // enum Status {
    //     Value(u32),
    //     Stop,
    // }
    //
    // let list_of_statuses: Vec<Status> = (0u32..20)
    //     .map(Status::Value)
    //     .collect();
    //
    // println!("list of statuses {:?}", list_of_statuses);

    // how call Returning Closures
    let f = returns_closure();
    let res = f(68);

    println!("res: {}", res);
    
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// fn add_one(x: i32) -> i32 {
//     x + 1    
// }
//
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

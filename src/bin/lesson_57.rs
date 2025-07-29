// 57: Hash set in Rust

use std::collections::HashSet;

fn main() {
    // 1)
    // let mut nums: HashSet<i32> = HashSet::new();
    // nums.insert(10); 
    // nums.insert(20); 
    // nums.insert(10); // will not insert a repeated value 
    // dbg!(nums);

    // 2)
    let mut nums = HashSet::from([10, 20, 10]);
    let vec = vec![1, 2, 3, 4, 4, 3, 5];
    // convert Vec to HasSet
    let mut hashed_vec: HashSet<i32> = vec.into_iter().collect();
    
    dbg!(nums, hashed_vec);

}

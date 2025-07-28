// 57: Hash set in Rust

use std::collections::HashSet;

fn main() {
    let mut nums: HashSet<i32> = HashSet::new();
    nums.insert(10); 
    nums.insert(10); 
    nums.insert(10); 
    dbg!(nums);


}

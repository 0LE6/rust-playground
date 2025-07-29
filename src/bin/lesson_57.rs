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
    // let mut nums = HashSet::from([10, 20, 10]);
    // let vec = vec![1, 2, 3, 4, 4, 3, 5];
    // // convert Vec to HasSet
    // let hashed_vec: HashSet<_> = vec.into_iter().collect();
    // 
    // dbg!(nums, hashed_vec);

    // 3) - contains
    // let mut nums = HashSet::from([1, 2, 3, 4, 4, 3, 5]);
    // dbg!(&nums);
    // dbg!(nums.contains(&69)); // false

    // 4) - remove
    let mut nums = HashSet::from([1, 2, 3, 4, 4, 3, 5]);
    // dbg!(&nums);
    // dbg!(nums.remove(&5)); // true
    //
    // let there_is: bool = nums.remove(&5); // false 'cause we already removed it
    //
    // dbg!(&nums, there_is);

    // 5) - length and is empty
    dbg!(nums.len(), nums.is_empty());

    // 6) - clear
    nums.clear();
    dbg!(&nums);

    nums.extend([1, 2, 3]); // other way of populate the Set
    dbg!(nums);
}

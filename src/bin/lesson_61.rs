// 61: generic functions in Rust

// Example 0: redundant code with duplicated logics
// (to solve in next example)
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 0];
    let characters = vec!['a', 'b', 'z', 'd', 'e'];

    // dbg!(get_fist_integer(&numbers));
    // dbg!(get_first_character(&characters));

    // Example 1: generics
    // dbg!(get_first(&numbers));
    // dbg!(get_first(&characters));
    
    // Example 2:
    dbg!(largest(&characters));
    dbg!(largest(&numbers));
}

// Example 2
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None
    }

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

// Example 1:  use generics
// just to see how generic work !!!
// fn get_first<T>(list: &[T]) -> &T {
//     &list[0]
// }

// part of the Example 0
// fn get_fist_integer(list: &[i32]) -> &i32 {
//     &list[0]
// }
//
// fn get_first_character(list: &[char]) -> &char {
//     &list[0]
// }

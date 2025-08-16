// 61: generic functions in Rust

// Example 0: redundant code with duplicated logics
// (to solve in next example)
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let characters = vec!['a', 'b', 'c', 'd', 'e'];

    // dbg!(get_fist_integer(&numbers));
    // dbg!(get_first_character(&characters));

    // Example 1: generics
    dbg!(get_first(&numbers));
    dbg!(get_first(&characters));
}

// Example 1:  use generics
fn get_first<T>(list: &[T]) -> &T {
    &list[0]
}

// part of the Example 0
fn get_fist_integer(list: &[i32]) -> &i32 {
    &list[0]
}

fn get_first_character(list: &[char]) -> &char {
    &list[0]
}

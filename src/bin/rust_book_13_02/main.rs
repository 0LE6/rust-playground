/*
* Processing a Series of Items with Iterators
*/

fn main() {
    // Initial example
    // https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-10
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    /*
    * iterators are lazy, meaning they have no effect until 
    * you call methods that consume the iterator to use it up
    */

    // https://doc.rust-lang.org/book/ch13-02-iterators.html#listing-13-11
    for val in v1_iter {
        println!("Got: {val}");
    }
}

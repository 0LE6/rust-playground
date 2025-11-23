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

    /*
    * https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators
    */
    let v2: Vec<i32> = vec![1, 2, 3];

    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    // assert_eq!(v3, vec![2, 3, 4]);
    dbg!(v3);

    println!("--------------------");

    // https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators
    // let v4: Vec<i32> = vec![1, 2, 3];
    // v4.iter().map(|x| x + 1);
     
}
/*
* https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method
*/
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    /*
    * https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-consume-the-iterator
    */
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
    

}



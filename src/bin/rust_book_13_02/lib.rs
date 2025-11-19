

/*
 * https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method
*/

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assset_eq!(v1_iter.next(), Some(&1));
    assset_eq!(v1_iter.next(), Some(&2));
    assset_eq!(v1_iter.next(), Some(&3));
    assset_eq!(v1_iter.next(), None);
}


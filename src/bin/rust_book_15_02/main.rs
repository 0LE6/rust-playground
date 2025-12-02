// Treating Smart Pointers Like 
// Regular References with "Deref"

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Unsafe Rust

use std::slice;

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    // let mut num = 5;
    //
    // let r1 = &raw const num;
    // let r2 = &raw mut num;
    //
    // unsafe {
    //     println!("{} - {}", *r1, *r2);
    // }
    //
    // let address = 0x012345usize;
    // let r = address as *const i32;
    //
    // unsafe {
    //     dangerous();
    // }

    // let mut v = vec![1, 2, 3, 4, 5, 6];
    //
    // let r = &mut v[..];
    //
    // let (a, b) = r.split_at_mut(3);
    //
    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    // let address = 0x012345usize;
    // let r = address as *mut i32;
    // 
    // let values: &[i32] =
    //     unsafe {
    //         slice::from_raw_parts_mut(r, 10000)
    //     };

    // unsafe {
    //     println!(
    //         "Absolute value of -3 accor. to C: {}",
    //         abs(-3)
    //     );
    // }

    // declaring abs() as safe we no longer
    // need to use unsafe
    // println!(
    //     "Absolute value of -3 accor. to C: {}",
    //     abs(-3)
    // );

    println!("value is: {HELLO_WORLD}");

}

unsafe extern "C" {
    // fn abs(input: i32) -> i32;
    safe fn abs(input: i32) -> i32;
}

fn split_at_mut(
    values: &mut [i32],
    mid: usize
) -> (&mut [i32], &mut [i32]) {

    let len = values.len();

    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(
                ptr.add(mid), len - mid
            ),
        )
    }
}

unsafe fn dangerous() { }

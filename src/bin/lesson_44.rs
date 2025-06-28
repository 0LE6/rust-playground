// 44: Error handling in Rust - time to panic!

fn main() {
    // let x: [i32; 2] = [1, 2];
    // println!("{}", x[2]);
    // panic!("Rusty ran away!");
    let v: Vec<i32> = vec![1, 2, 3];
    v[99];
}

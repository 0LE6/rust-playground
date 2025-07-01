// 36: Structs provide clarity in Rust

fn main() {
    let rect = (20, 30); // emulating x & y
    dbg!(get_area(rect));
}

fn get_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

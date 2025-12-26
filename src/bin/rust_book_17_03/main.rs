// Yielding Control to the Runtime

use std::{thread, time::Duration};

fn main() {
    
}

fn slow(name: &str, ms: u64) {
    thread::sleep(
        Duration::from_millis(ms)
    );
    println!("'{name}' ran for {ms} ms!")
}

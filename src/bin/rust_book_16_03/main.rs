// Shared-State Concurrency

use std::{sync::Mutex, thread};

fn main() {
    // let m = Mutex::new(5);
    //
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    //
    // println!("m = {m:?}");

    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 1..10 {
        let handle = thread::spawn(
            move || {
                let mut num = counter
                    .lock()
                    .unwrap();

                *num += 1;
            }
        );
        handles.push(handle);
    }

    for hanlde in handles {
        hanlde.join().unwrap();       
    }

    println!(
        "Result: {}",
        *counter.lock().unwrap()
    );
}

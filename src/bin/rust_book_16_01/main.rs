// Using Threads to Run Code Simultaneously

use std::{thread, time::Duration};

fn main() { 
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!(
                "hi number {i} from the spawned thread!"
            );
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_secs(1));
    }
    
    handle.join().unwrap();
}

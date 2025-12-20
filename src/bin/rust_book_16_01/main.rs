// Using Threads to Run Code Simultaneously

use std::{thread, time::Duration};

fn main() { 
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!(
    //             "hi number {i} from the spawned thread!"
    //         );
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    //
    // handle.join().unwrap(); // JoinHandle<()>
    //
    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_secs(1));
    // }

    // join makes sure the spawned thread 
    // finishes before main exits.   
    // handle.join().unwrap(); // JoinHandle<()>
    // main thread waits because of the
    // call to handle.join() and does not 
    // end until the spawned thread is finished.

    let v = vec![1, 2, 3];

    // By adding the move keyword before the
    // closure, we force the closure to take 
    // ownership of the values it’s using rather
    // than allowing Rust to infer that it 
    // should borrow the values.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // drop(v);
    // If Rust allowed us to run this code, 
    // there’s a possibility that the spawned 
    // thread would be immediately put in 
    // the background without running at all. 

    handle.join().unwrap();



}

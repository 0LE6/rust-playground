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

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();



}

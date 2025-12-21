// Transfer Data Between Threads with Message Passing

use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}


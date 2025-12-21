// Transfer Data Between Threads with Message Passing

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}

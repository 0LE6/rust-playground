pub mod accounts;
pub mod transactions;

pub fn announce(msg: &str) {
    println!("[BANK ACCOUNT] {msg}");
}

// https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
// src/bin/bank.rs (seems a modern way)
// src/bin/bank/mod.rs (older style, still supported path)

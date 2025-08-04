pub mod accounts;
pub mod transactions;

pub fn announce(msg: &str) {
    println!("[BANK ACCOUNT] {msg}");
}

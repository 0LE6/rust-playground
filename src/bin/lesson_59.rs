// 59: modules in Rust
mod bank;

fn main() {
    let mut acc = bank::accounts::Account::new("Rusty");
    println!("[ACCOUNT] Created: {:?}", acc);
}

// 59: modules in Rust
mod bank;

fn main() {
    let mut acc = bank::accounts::Account::new("Rusty");
    println!("[ACCOUNT] Created: {:?}", acc);

    bank::transactions::deposit(&mut acc, 150);
    bank::transactions::withdraw(&mut acc, 20);

    println!("[ACCOUNT] Final state: {:?}", acc);

    bank::announce("Maintanance at 01:30 PM");
}

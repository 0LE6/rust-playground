// 60: create own libray in Rust
use rust_playground::banking::{accounts, transactions};

fn main() {
    let mut rusty = accounts::open_account(1);
    let mut frosty = accounts::open_account(2);

    dbg!(&rusty, &frosty);
}

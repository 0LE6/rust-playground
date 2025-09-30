// from Rust book
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, mate!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query , file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.file_path)?;
    
    println!("\nWith the following text: \n {content}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three!";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));;
    }
}





// pub mod banking {
//     pub mod accounts {
//         #[derive(Debug)]
//         pub struct Account {
//             pub account_number: u32,
//             pub balance: f64,
//         }
//
//         pub fn open_account(id: u32) -> Account {
//             println!("Account {} opened!", id);
//             Account { account_number: (id), balance: (0.0) }
//         }
//
//         #[allow(dead_code)]
//         fn close_account(account: &mut Account) {
//             println!("Account {} closed!", account.account_number);
//             account.balance = 0.0;
//         }
//     }
//
//     pub mod transactions {
//         // super allows us accese to functionality outside
//         // the "transactions" module
//         use super::accounts::Account;
//
//         pub fn deposit(account: &mut Account, amount: f64) {
//             account.balance += amount;
//             println!(
//                 "Deposited ${:.2} into Account {}. New balance: {:.2}",
//                 amount, account.account_number, account.balance
//             );
//         }
//
//         pub fn withdraw(acc: &mut Account, amount: f64) {
//             if amount > acc.balance {
//                 println!(
//                     "[TRANSACTION] ERROR: Insufficient funds for ${} withdraw from account 
//                     {} with balance ${:.2}",
//                     amount, acc.account_number, acc.balance
//                 );
//             } else {
//                 acc.balance -= amount;
//                 println!(
//                     "[TRANSACTION] Withdraw ${:.2} from Account {}. New balance: ${:.2}",
//                     amount, acc.account_number, acc.balance
//                 );
//             }
//         }
//
//         pub fn transfer(from: &mut Account, to: &mut Account, amount: f64) {
//             if from.balance >= amount {
//                 from.balance -= amount;
//                 to.balance += amount;
//                 println!(
//                     "Transferred ${:.2} from Account {} to Account {}.",
//                     amount,from.account_number, to.account_number
//                 );
//             } else {
//                 println!(
//                     "Transfer failed: Account {} has insufficient funds (${:.2})",
//                     from.account_number, from.balance
//                 );
//             }
//         }
//     }
// }
 


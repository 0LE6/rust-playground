pub mod banking {
    pub mod accounts {
        #[derive(Debug)]
        pub struct Account {
            pub account_number: u32,
            pub balance: f64,
        }

        pub fn open_account(id: u32) -> Account {
            println!("Account {} opened!", id);
            Account { account_number: (id), balance: (0.0) }
        }

        #[allow(dead_code)]
        fn close_account(account: &mut Account) {
            println!("Account {} closed!", account.account_number);
            account.balance = 0.0;
        }
    }

    pub mod transactions {
        // super allows us accese to functionality outside
        // the "transactions" module
        use::super::accounts::Account;

        pub fn deposit(account: &mut Account, amount: f64) {
            account.balance += amount;
            println!(
                "Deposited ${:.2} into Account {}. New balance: {:.2}",
                amount, account.account_number, account.balance
            );
        }

        pub fn withdraw(acc: &mut Account, amount: f64) {
            if amount > acc.balance {
                println!(
                    "[TRANSACTION] ERROR: Insufficient funds for ${} withdraw from account 
                    {} with balance ${:.2}",
                    amount, acc.account_number, acc.balance
                );
            } else {
                acc.balance -= amount;
                println!(
                    "[TRANSACTION] Withdraw ${:.2} from Account {}. New balance: ${:.2}",
                    amount, acc.account_number, acc.balance
                );
            }
        }
    }
}

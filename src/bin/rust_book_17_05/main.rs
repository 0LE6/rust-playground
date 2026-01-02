// A Closer Look at the Traits for Async

use std::time::Duration;
use std::pin::{Pin, pin};

fn main() {
    trpl::block_on(async {
        // let fut1 = async {
        //     for i in 1..10 {
        //         println!(
        //             "Hi number {i} from the 1st task!"
        //         );
        //         trpl::sleep(
        //             Duration::from_millis(500)
        //         ).await
        //     }
        // };
        //
        // let fut2 = async {
        //     for i in 1..5 {
        //         println!(
        //             "Hi number {i} from the 2nd task!"
        //         );            
        //         trpl::sleep(
        //             Duration::from_millis(500)
        //         ).await
        //     }
        // };
        //
        // trpl::join(fut1, fut2).await;

        let (tx, mut rx) = trpl::channel();

        // let val = String::from("hi");
        // tx.send(val).unwrap();
        //
        // let received = rx.recv().await.unwrap();
        // println!("received '{received}'");
    
        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
               tx1.send(val).unwrap();
               trpl::sleep(
                   Duration::from_millis(500)
               ).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });
        
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
               tx.send(val).unwrap();
               trpl::sleep(
                   Duration::from_millis(500)
               ).await;
            }
        });
        
        let futures: 
            Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures);
    });

}

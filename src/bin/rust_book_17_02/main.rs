// Applying Concurrency with Async

use std::time::Duration;

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
    
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
               tx.send(val).unwrap();
               trpl::sleep(
                   Duration::from_millis(500)
               ).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

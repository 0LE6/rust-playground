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

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
}

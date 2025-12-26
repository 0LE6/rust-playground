// Yielding Control to the Runtime

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // let one_ms = Duration::from_millis(1);
        // let a = async {
        //     println!("'a' started");
        //     slow("a", 30);
        //     trpl::yield_now().await;
        //     slow("a", 10);
        //     trpl::yield_now().await;
        //     slow("a", 20);
        //     trpl::yield_now().await;
        //     println!("'a' finished!");
        // };
        //
        // let b = async {
        //     println!("'b' started");
        //     slow("b", 75);
        //     trpl::yield_now().await;
        //     slow("b", 10);
        //     trpl::yield_now().await;
        //     slow("b", 15);
        //     trpl::yield_now().await;
        //     slow("b", 350);
        //     trpl::yield_now().await;
        //     println!("'b' finished!");
        // };
        //
        // trpl::select(a, b).await;

        let slow = async {
            trpl::sleep(
                Duration::from_secs(5)
            ).await;
            "Finally finished!"
        };

        match timeout(
            slow,
            Duration::from_secs(2)
        ).await {
            Ok(msg) => println!("OK with '{msg}'"),
            Err(duration) => {
                println!(
                    "FAIL after {} secs", 
                    duration.as_secs()
                );          
            }
        }
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(
        Duration::from_millis(ms)
    );
    println!("'{name}' ran for {ms} ms!")
}

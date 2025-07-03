// Lets do some of the same tasks we did in the threads sections, but now in async
// First counting up

use std::time::Duration;

fn main() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // We even need the join thingy to wait for the first task to finish
        handle.await.unwrap();

        // Channels are also similar
        // And also multiple transmitter, single receiver
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            // We move tx into this block so tx is dropped when the block
            // ends and the channel is closed so we arent permanently blocked
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("Received `{value}`");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

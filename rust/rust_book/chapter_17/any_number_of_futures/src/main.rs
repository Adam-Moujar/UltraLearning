// If we wanted to join more than 2 features, say 3 features. We have to use
// join3(future1, future2, future3) rather than join, and that just sucks.
//
// Thankfully, we have join!, a macro that takes in an arbitrary number of futures for us to join,
// so, crisis averted.
//
// Except not really, this only works if we know the futures ahead of time,
// we have a more elegant version using vectors
use std::pin::Pin;
use std::thread;
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        // join_all works in vectors so lets make a vector of the futures
        // Except we run into a problem, rust compiler tells us that async blocks have different
        // types !? And so we cant have them in the same vector because vector elements have the same type.
        // The compiler does give us a fix of wrapping stuff in a box pin, which is what we have
        // below
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

        trpl::join_all(futures).await;

        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
        // We cant use join_all here because join_all needs the vec to have the same return type
        // for the async blocks, in this case they arent, one is a u32, the other is a &str and the
        // last one is a bool.
        // This is the trade off between join! and join_all
        //
        //
        // A problem with async is that control is only being handed to runtime during an await
        // handoff
        // That means that a async task can hog operations unless it awaits to give another task
        // a chance to run.
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });
    // In here a will run until the first await, then switch to running b until it hits its await,
    // go back and finish a, and then finish b
}
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

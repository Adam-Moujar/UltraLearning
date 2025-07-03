// Streams are a collection of futures.
// And like collections, they work very similarly to iterators
// So much so we can convert iterators to streams
//
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        // Using channels as essentially iterators
        // Not very useful yet, since we already have iterators
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|x| x * 2);
        let mut stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);
        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }

        // Gets the stream so we can iterate over the messages
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }

        // We can also merge streams
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100)) // this is here because the rate of intervals is
            // much faster than the rate of messages so it will be hard to see both, so we throttle
            // the intervals so we have easier time to see
            .timeout(Duration::from_secs(10)); // have to convert from stream of numbers to stream
        // of integers to merge with messages since they need to be of the same type
        let merged = messages.merge(intervals).take(20); // merging them
        let mut stream = pin!(merged); // have to be pinned
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    // Transforms a receiver into a stream
    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

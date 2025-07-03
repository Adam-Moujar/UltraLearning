## Notes

Now that we have learned 2 approaches to concurrency in threads and async, the question
becomes which one to choose? And as with many things in life, the answer is it depends. 
Sometimes it might even be beneficial to choose a hybrid approach. 

Threads come with a big of memory overhead and a cost when starting up and shutting down. 
And it's intrinsically connected to the OS, so systems without an OS have no threads to work with.

Async is complementary, it's handled by the runtime, it does not require threads and rather run on tasks. 

Many of the things one can do with async, can also be down in threads, for example in the listing 17-40
in rust book, we used trpl::spawn_task to handle a task using async method, however he could use threads instead.

```
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // This is *not* `trpl::spawn` but `std::thread::spawn`!
    thread::spawn(move || {
        let mut count = 0;
        loop {
            // Likewise, this is *not* `trpl::sleep` but `std::thread::sleep`!
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
```

One advantage threads has over async is its simplicity. Coding with threads involved less headache in my opinion
that coding with async, however there are very real advantages to async. They are more flexible in the way they can
switch between tasks, the range of work they can do is much more wide. The future trait is so rich and supported
that building complex stuff with async is much easier than with threads. 

There are some rule of thumbs to help us when choosing whether to make it async or thread:

- If the work is very parallelizable, the work can be done independently, threads is a better choice.
- If the work is very concurrent, we need to swap tasks often and the tasks can come from different sources,
at different intervals and with different rates, async is a better choice. 

One common place to use threads is in graphics, since the tasks are very well defined and can be done independtly,
while async is used in the web, where requests need to be handled flexibly and might require a lot more flexibility.

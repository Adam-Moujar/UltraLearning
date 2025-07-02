// Messaging passing is fine but there are cases where it either does not work,
// or requires heavy work to make it fit.
//
// Another method we have availabe is that threads access the same shared data.
// We have mutexes (wow), they are shared data that locks when a thread accesses them
// and makes sure that only a thread has access at a time.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(5)); // Creating a mutex
    {
        let mut num = m.lock().unwrap(); // Getting the lock of the mutex so we can change it
        *num = 6;
    } // num goes out of scope, drop is called and lock is returned, so now it can be locked again
    println!("m = {m:?}"); // The main thread cant do this because we got the lock of m 
    //
    // Lets use it to increment a value using 10 thread
    let counter = Arc::new(Mutex::new(0)); // we need this because we cant move counter into the
    // threads and then use it out of it, so we could use something like rc to make the threads own
    // counter aswell as the main thread and then they could use it, but the problem with that is
    // that rc is not thread safe, arc is the thread safe version of it, comes with performance
    // hits however, and so thats why we use it
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Let the thread also own counter by increment arc
        // count
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    // Mutex does come with problems
    // First like refcell, we just mutated a immutable variable
    // counter is immutable and yet we have added 10 to it.
    // 2, just like Rc, we can create a lock situation, often called a deadlock
    // when 2 things need each other but a 2 threads holds a key of the of the other. They will
    // wait on each other to release the lock forever.
    //
    // Fearless concurrency is true to its name.
    // Its not fear free concurrency, just fear less concurrency
}

// Veeeeery similar to Rc, but when we need to share between threads, we use Arc.
// The idea is that Arc comes with some overhead which is why its preferable to use Rc and when you
// need to share some data between threads, Arc is the choice
use std::sync::Arc;
use std::thread;
use std::time::Duration;
fn main() {
    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a
        // reference in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }

    // Make sure all Arc instances are printed from spawned threads.
    thread::sleep(Duration::from_secs(1));
}

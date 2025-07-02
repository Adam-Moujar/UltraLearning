// We are onto concurrency.
// Threads run independent processes.
// We can split our program into threads and allow the os to make our program concurrent
// So long as we have multiple cores.
// There are things to watch out with concurrency, race conditions and deadlocks.
//
// Anyway lets get into it
use std::thread; // allow threads
use std::time::Duration;

fn main() {
    // We create a thread and give it a task.
    //thread::spawn(|| {
    //    for i in 1..10 {
    //        println!("hi number {i} from the spawned thread!");
    //        thread::sleep(Duration::from_millis(1)); // This is here to make sure the side thread
    //        // takes a break
    //    }
    //});

    //// Main thread just continues along
    //for i in 1..5 {
    //    println!("hi number {i} from the main thread!");
    //    thread::sleep(Duration::from_millis(1));
    //}

    // The above code has a problem. If the main thread ends, so do the offshot threads. So it
    // could be the case that the thread ends prematurely, in fact thats what happens.
    // If we get unlucky enough, or the os scheduler is in a spicy mood, it might not even run at
    // all.
    //
    // .join() fixes that

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // This makes sure that the main threads waits until the side thread is
    // done and joins back with it before continuing, in this case, continuing just ends the
    // program.
    //
    // Now its important to note that the thread we created gets a closure passed in with no
    // parameters passed. It is often the case that we want to use variablse from the environment
    // In those cases, we have to move the variables to the thread so they get captured
    let v = vec![1, 2, 3];

    //let handle = thread::spawn(|| {
    //    println!("Here's a vector: {v:?}");
    //});

    //drop(v); // oh no!
    //
    //This is not allowed because if the drop(v) runs before the thread then that means the thread
    //would be running using dropped data and thats a nono.
    //This is the correct code:
    let v = vec![1, 2, 3];

    // We added a move
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    //drop(v); // This would be a compiler error since v is now owned by the thread

    handle.join().unwrap();
}

// Sometimes we want to send data between threads.
// We can do this using channel, whose purpose is to literally send data between threads.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    // Transmitter and receiver
    // Transmitter sends data
    // Receivers receive data
    //
    // We can have many transmitters, only 1 receiver
    // If transmitters drops, channel drops
    // If the one receiver drops, the channel drops

    // We give a thread the tx, transmitter, and have it send a string saying hi
    //thread::spawn(move || {
    //    let val = String::from("hi");
    //    tx.send(val).unwrap();
    //    // println!("val is {val}"); // This doesnt work because send takes ownership of val
    //});

    //// have receiver receive the message.
    //let received = rx.recv().unwrap(); // This will actually wait until it receives a message,
    //// which if it never does, bad news
    //println!("Got {received}");

    // we can also send multiple values and receive multiple values
    //thread::spawn(move || {
    //    let vals = vec![
    //        String::from("hi"),
    //        String::from("from"),
    //        String::from("tha"),
    //        String::from("thread"),
    //    ];
    //    for val in vals {
    //        tx.send(val).unwrap();
    //        thread::sleep(Duration::from_secs(1));
    //    }
    //});
    //for received in rx {
    //    println!("Got {received}");
    //}

    let tx1 = tx.clone(); // Create another receiver
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("tha"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {received}");
    }
}

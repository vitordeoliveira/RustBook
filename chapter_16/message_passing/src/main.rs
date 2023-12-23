use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    // The spawned thread needs to own the transmitter to be able to send messages through the channel
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // NEXT
    // Sending Multiple Values and Seeing the Receiver Waiting
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }

    // NEXT
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("Hello world");
        tx.send(s.clone()).unwrap();
        tx.send(s).unwrap();
    });
    let s = rx.recv().unwrap();
    let n = rx.recv().unwrap();
    println!("{s} {n}");
}

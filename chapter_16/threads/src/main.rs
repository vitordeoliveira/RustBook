use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // NEXT
    let v = vec![1, 2, 3];

    // We force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing Rust that the main thread won’t use v anymore
    // drop(v);

    handle.join().unwrap();

    // NEXT
    // Context: The move keyword causes n to be copied into the closure, so the assignments n = n + 1 within thread::spawn have no effect on the outer n.
    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        thread::spawn(move || {
            n = n + 1;
        })
    });
    n = n + 1;
    t.join().unwrap().join().unwrap();
    println!("{n}");
}

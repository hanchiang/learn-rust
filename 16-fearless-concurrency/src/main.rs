mod channel;
mod mutex;

use std::thread;
use std::time::Duration;

fn main() {
    basic_thread();
    move_closure();
    channel::main();
    mutex::main();
}

// Note that with this function, the new thread will be stopped when the main thread ends, whether or not it has finished running.
// The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.
// The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads.
fn basic_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
    handle.join().unwrap();
}

fn move_closure() {
    let v = vec![1, 2, 3];

    // By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
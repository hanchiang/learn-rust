use std::sync::{Mutex, Arc};
use std::thread;

pub fn main() {
    single_thread();
    multiple_threads();
}

fn single_thread() {
    let m = Mutex::new(5);

    {
        // To access the data inside the mutex, we use the lock method to acquire the lock.
        // This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
        // The call to lock would fail if another thread holding the lock panicked.
        // In that case, no one would ever be able to get the lock, so we’ve chosen to unwrap and have this thread panic if we’re in that situation.
        // The call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10
    {
        let counter = Arc::clone(&counter);
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
}
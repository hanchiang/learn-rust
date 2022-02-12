use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    basic();
    send_multiple_values();
    multiple_producers();
}

fn basic() {
    // mpsc stands for multiple producer single consumer
    // The mpsc::channel function returns a tuple, the first element of which is the sending end and the second element is the receiving end.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // The send method returns a Result<T, E> type, so if the receiving end has already been dropped and there’s nowhere to send a value,
        // the send operation will return an error.
        tx.send(val).unwrap();
    });

    // The receiving end of a channel has two useful methods: recv and try_recv.
    // We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel.
    // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately:
    // an Ok value holding a message if one is available and an Err value if there aren’t any messages this time.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn send_multiple_values() {
    let (tx, rx) = mpsc::channel();

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

    // we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator.
    // For each value received, we’re printing it. When the channel is closed, iteration will end.
    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producers() {
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
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
#![allow(dead_code)]
use std::sync::mpsc;
use std::sync::Arc;

#[derive(Debug)]
struct Message {
    title: String,
    message: String
}

use std::{thread, time};

pub fn thread_messages_test() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    let t1 = thread::spawn(move || {
        let val = Message {
            title: "how are you ?".to_string(),
            message: "This is testing the message body.".to_string()
        };
        tx.send(val).unwrap();
    });

    let t2 = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("Thread 2 : Got: {:?}", received);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

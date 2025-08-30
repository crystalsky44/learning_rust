use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
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

    for recieved in rx {
        println!("Got: {recieved}");
    }
}

// Quizes
use std::{sync::mpsc, thread};

    enum ClientMessage { Incr, Get, Quit }
    enum ServerMessage { Get(usize) }

fn main() {
    let (server_tx, client_rx) = mpsc::channel();
    let (client_tx, server_rx) = mpsc::channel();
    let server = thread::(move || {
        let mut n = 0;
        loop {
            match server_rx.recv().unwrap() {
                ClientMessage::Quit => break,
                ClientMessage::Incr => n += 1,
                ClientMessage::Get => server_tx.send(ServerMessage::Get(n)).unwrap()
            }
        }
    });

    for msg in [ClientMessage::Incr, ClientMessage::Get, ClientMessage::Quit] {
        client_tx.send(msg).unwrap();
    }

    if let ServerMessage::Get(n) = client_rx.recv().unwrap() {
        println!("{}", n)
    }

    server.join().unwrap();
}

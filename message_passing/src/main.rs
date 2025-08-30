use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("hi");
        tx.send(value).unwrap();
        println!("value is {value}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

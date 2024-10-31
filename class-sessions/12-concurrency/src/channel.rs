use std::sync::mpsc;
use std::thread::{self};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<u8>();

    let sender = thread::spawn(move || sender(tx));
    let adder = thread::spawn(move || adder(rx));

    sender.join().unwrap();
    adder.join().unwrap();
}

fn sender(tx: mpsc::Sender<u8>) {
    let quarter_second = Duration::from_millis(250);
    for i in 1..11 {
        tx.send(i).expect("Receiver is dead");
        println!("Sent: {}", i);
        thread::sleep(quarter_second);
    }
}

fn adder(rx: mpsc::Receiver<u8>) {
    let mut processed: u8;
    for received in rx {
        processed = received * 2;
        println!("Processed: {}", processed);
    }
}

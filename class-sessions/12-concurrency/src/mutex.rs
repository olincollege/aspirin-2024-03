use std::sync::{Arc, RwLock};
use std::thread::{self};
use std::time::Duration;

fn main() {
    let curr = Arc::new(RwLock::new(0));
    let curr2 = Arc::clone(&curr);

    let sender = thread::spawn(move || sender(curr));
    let adder = thread::spawn(move || adder(curr2));

    sender.join().unwrap();
    adder.join().unwrap();
}

fn sender(num: Arc<RwLock<u8>>) {
    let quarter_second = Duration::from_millis(250);
    for i in 1..11 {
        let mut curr = num.write().unwrap();
        *curr = i;
        drop(curr);

        println!("Sent: {}", i);
        thread::sleep(quarter_second);
    }
    panic!()
}

fn adder(pair: Arc<RwLock<u8>>) {
    let mut processed: u8;
    loop {
        let curr = pair.read().unwrap();
        processed = *curr * 2;
        drop(curr);
        println!("Processed: {}", processed);
    }
}

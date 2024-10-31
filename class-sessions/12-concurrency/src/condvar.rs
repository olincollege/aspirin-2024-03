use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self};
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(0u8), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let sender = thread::spawn(move || sender(pair));
    let adder = thread::spawn(move || adder(pair2));

    sender.join().unwrap();
    adder.join().unwrap();
}

fn sender(pair: Arc<(Mutex<u8>, Condvar)>) {
    let quarter_second = Duration::from_millis(250);
    for i in 1..11 {
        let (lock, cvar) = &*pair;
        let mut curr = lock.lock().unwrap();
        *curr = i;
        cvar.notify_one();
        drop(curr);

        println!("Sent: {}", i);
        thread::sleep(quarter_second);
    }
}

fn adder(pair: Arc<(Mutex<u8>, Condvar)>) {
    let mut processed: u8;
    loop {
        let (lock, cvar) = &*pair;
        let mut curr = lock.lock().unwrap();
        while *curr == 0 {
            curr = cvar.wait(curr).unwrap();
        }
        processed = *curr * 2;
        *curr = 0;
        println!("Processed: {}", processed);
    }
}

use std::thread;
use std::time;
use std::sync::{Arc,RwLock};

/// Simple receiver function, use the RwLock to get data
/// whenever it can.
fn receiver (reader_id : u32, lock : Arc<RwLock<u32>>) {
    let mut number : u32 = 0;
    let wait_ms = time::Duration::from_millis(500);
    loop {
        if number == 10 {
            return
        }
        {
            let read_value = lock.read().unwrap();
            number = *read_value;
        }
        println!("sender {:?}, got value {:?}", reader_id, number);
        thread::sleep(wait_ms);
    }
}

// Sender: acquires the rwlock and send a new data
fn sender (lock : Arc<RwLock<u32>>) {
    let thousands_ms = time::Duration::from_millis(1000);
    let mut number  : u32 = 0;
    for _i in 0 .. 20 {
        println!("hello from sender {:?}", number);
        {
            let mut w = lock.write().unwrap();
            *w = number;
        }
        number = number + 1;
        thread::sleep(thousands_ms);
    }
}

fn main() {
    let rwlock = Arc::new(RwLock::new(0));
    let bla = rwlock.clone();
    let sender_thread = thread::spawn( move || { sender(bla);});
    let bla = rwlock.clone();
    let receiver_thread = thread::spawn( move || { receiver(1, bla);});
    let bla = rwlock.clone();
    let receiver_thread2 = thread::spawn( move || { receiver(2, bla);});
   

    sender_thread.join().expect("sender finished");
    receiver_thread.join().expect("receiver finished");
    receiver_thread2.join().expect("receiver finished");
}

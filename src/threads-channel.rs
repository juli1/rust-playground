use std::thread;
use std::time;
use std::sync::mpsc;

fn receiver (r : &mpsc::Receiver<u32>) {
    let thousands_ms = time::Duration::from_millis(1000);
    for i in 0 .. 10 {
        let number = r.recv().unwrap();
        println!("hello from receiver {:?}", number);
    }
}


fn sender (s : &mpsc::Sender<u32>) {
    let thousands_ms = time::Duration::from_millis(1000);
    let mut number : u32 = 0;
    for i in 0 .. 10 {
        println!("hello from sender {:?}", number);
        number = number + 1;
        s.send(number).unwrap();
        thread::sleep(thousands_ms);
    }
}

fn main() {
    /// Create the communication channel.
    let (tx, rx) = mpsc::channel();

    let sender_thread = thread::spawn( move || { sender(&tx);});
    let receiver_thread = thread::spawn( move || { receiver(&rx);});
   

    sender_thread.join();
    receiver_thread.join();
}

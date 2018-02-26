
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time;

const NBPLAYERS : u32 = 6;

fn main () {
    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let mut threads = Vec::new();
    let thousands_ms = time::Duration::from_millis(1000);

    for n in 0 .. NBPLAYERS {
        let newpair = pair.clone();

        let thr = thread::spawn(move|| {
            let &(ref lock, ref cvar) = &*newpair;
            {
                let mut started = lock.lock().unwrap();
                *started = *started + 1;
                cvar.notify_all();
            }
            loop {
                let started = lock.lock().unwrap();
                if *started == NBPLAYERS {
                    break;
                }
                println!("Thread {} wait!", n);
                cvar.wait(started).expect("bla");
            }
            println!("Thread {} started!", n);
        });

        threads.push(thr);
        thread::sleep(thousands_ms);

    }

    for thr in threads {
        thr.join().unwrap();
    }
   
}


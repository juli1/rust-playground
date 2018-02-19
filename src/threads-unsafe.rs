use std::thread;
use std::time;

static mut glob : u32 = 0;

fn handler () {
    let thousands_ms = time::Duration::from_millis(1000);
    for i in 0 .. 10 {
        unsafe {
            println!("hello {:?}, global={:?}", i, glob);
            glob = glob + 1;
        }
        thread::sleep(thousands_ms);
    }
}

fn main() {
    let mut threads = vec![];
    for _i in 1 .. 3 {
        threads.push(thread::spawn( move || { handler();}));
    }

    for t in threads {
        let _ = t.join();
    }
}

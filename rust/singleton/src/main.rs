use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use std::{mem, thread};

// From: https://stackoverflow.com/a/27826181/7585591

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<u8>>,
}

fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;

    // Once will make sure the routine will only called in the program lifetime once
    // no matter how often it is called.
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(0)),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

fn main() {
    // Let's use the singleton in a few threads
    let threads: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 10));
                let s = singleton();
                let mut data = s.inner.lock().unwrap();
                *data = i as u8;
            })
        })
        .collect();

    // Read the current state of the singleton
    for _ in 0u8..20 {
        thread::sleep(Duration::from_millis(5));

        let s = singleton();
        let data = s.inner.lock().unwrap();
        println!("It is: {}", *data);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}

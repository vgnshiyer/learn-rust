use std::{sync::{Arc, Mutex, RwLock}, thread, time::Duration};

fn main() {
    // to share data across multiple thread, use Arc
    // Arc is a shared pointer -> has some overhead (reference counter) compared to standard pointer
    // Arc itself lets you only use the data across threads, but they are still immutable
    // we make them mutable with Mutex
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap(); // lock the data by a thread
            data.push(i);
            // the lock is realeased here
        });
        handles.push(handle);
    }

    // threads will run in a random order
    for handle in handles {
        handle.join().unwrap(); // wait for threads to complete
    }

    println!("Print data in main: {:?}", data.lock().unwrap());

    println!("");
    println!("------------------------------------------------");
    println!("");

    // we can use RwLock to have read and write locks on the data
    // threads can simultaneously read without locks, but the write has to happen with a lock
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            if i == 2 {
                let mut data = data_clone.write().unwrap(); // write lock -> everything has to stop
                println!("Writing to data from thread {}", i);
                data.push(i);
                thread::sleep(Duration::from_secs(1));
                println!("Done writing from thread {}", i)
                // the write lock is released here
            } else {
                // simulataneous read
                let data = data_clone.read().unwrap();
                println!("Reading data in thread {} {:?}", i, data);
                thread::sleep(Duration::from_secs(1));
                println!("Done reading data from thread {}", i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // wait for threads to complete
    }

    println!("Print data in main: {:?}", data.read().unwrap());

}

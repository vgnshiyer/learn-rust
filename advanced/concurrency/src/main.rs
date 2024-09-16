use std::{sync::{Arc, Mutex}, thread};

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
}

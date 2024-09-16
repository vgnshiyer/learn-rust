use std::{thread, sync::Arc};

fn main() {
    // to share data across multiple thread, use Arc
    // Arc is a shared pointer -> has some overhead (reference counter) compared to standard pointer
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Print data in thread {} {:?}", i, data_clone);
        });
        handles.push(handle);
    }

    // threads will run in a random order
    for handle in handles {
        handle.join().unwrap(); // wait for threads to complete
    }

    println!("Print data in main: {:?}", data);
}

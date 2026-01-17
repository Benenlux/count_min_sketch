mod min_sketch;

use std::{sync::Arc, thread};

use crate::min_sketch::CountMinSketch;

fn main() {
    let cms = CountMinSketch::new(4, 3);

    let shared_cms = Arc::new(cms);
    let mut handles = vec![];

    for _ in 0..10 {
        let cms_clone = Arc::clone(&shared_cms);
        let handle = thread::spawn(move || {
            cms_clone.insert("192.168.0.1");
            cms_clone.insert("127.0.0.1");
        });
        handles.push(handle);
    }
    shared_cms.insert("192.168.0.1");
    shared_cms.insert("192.168.0.1");
    for handle in handles {
        handle.join().unwrap();
    }
    let count = shared_cms.count("192.168.0.1");
    println!("{:?}", count);
    println!("{}", shared_cms);
}

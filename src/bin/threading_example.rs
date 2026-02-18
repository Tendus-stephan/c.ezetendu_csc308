use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let mut handles = vec![];
    
    // Create 3 threads
    for i in 1..=3 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            for j in 1..=5 {
                let message = format!("T{}: Message {}", i, j);
                tx.send(message).unwrap();
            }
        });
        handles.push(handle);
    }
    
    // Drop the original sender so the channel closes when all threads are done
    drop(tx);
    
    // Main thread receives and prints messages
    for received in rx {
        println!("{}", received);
    }
    
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

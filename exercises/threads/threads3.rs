// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// AI

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: mpsc::Sender<u32>) {
    let q1 = Arc::clone(&q);
    let q2 = Arc::clone(&q);

    // First thread: sending values from the first_half of the queue
    let tx1 = tx.clone(); // Clone the sender for the first thread
    thread::spawn(move || {
        let q = q1.lock().unwrap();
        for val in &q.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Second thread: sending values from the second_half of the queue
    let tx2 = tx.clone(); // Clone the sender for the second thread
    thread::spawn(move || {
        let q = q2.lock().unwrap();
        for val in &q.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let queue_length = queue.lock().unwrap().length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("Total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}


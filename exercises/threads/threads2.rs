// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// AI
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Lock the Mutex to safely mutate the shared data
            let mut status_locked = status_shared.lock().unwrap();
            status_locked.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the value of the JobStatus.jobs_completed after all threads have finished
    // Since we have safely mutated the data using Mutex, we can lock it here again to read the value
    let status_locked = status.lock().unwrap();
    println!("jobs completed {}", status_locked.jobs_completed);
}

